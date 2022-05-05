use std::alloc::alloc;
use std::alloc::Layout;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::Entry;
use crate::EntryResult;
use crate::RawHashTable;
use crate::Remove;

use super::EntryBucket;

impl<K, V> EntryBucket<K, V> {
    pub fn alloc(size: usize) -> *mut Self {
        let layout = match Layout::array::<Self>(size) {
            Ok(layout) => layout,
            Err(_) => panic!("Cannot initialize EntryBuckets"),
        };

        // allocate and init with None
        unsafe {
            let raw = alloc(layout) as *mut Self;

            for i in 0..size {
                *raw.add(i) = EntryBucket::None;
            }

            raw
        }
    }
}

pub struct LinearProbing<K, V> {
    step: usize,
    _marker: PhantomData<(K, V)>,
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for LinearProbing<K, V> {
    fn default() -> Self {
        LinearProbing {
            step: 1,
            _marker: PhantomData,
        }
    }

    fn entry(&self, table: &RawHashTable, key: &K, hash: u64) -> EntryResult<EntryBucket<K, V>> {
        let mut index = hash as usize & table.mask;

        let initial_bucket =
            unsafe { (table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>).add(index) };

        let mut bucket = initial_bucket;

        loop {
            match unsafe { &*bucket } {
                EntryBucket::None | EntryBucket::Tombstone => {
                    return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                }
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return EntryResult::Some(NonNull::new(bucket as *mut _).unwrap());
                    }

                    index = (index + self.step) & table.mask;
                    unsafe { bucket = bucket.add(index) }
                }
            }

            if bucket == initial_bucket {
                return EntryResult::Full;
            }
        }
    }
}

pub struct LinearProbingRemoval {}

impl<K: PartialEq> Remove<K> for LinearProbingRemoval {
    fn default() -> Self {
        todo!()
    }

    fn remove<T>(&self, table: &mut RawHashTable, key: &K, hash: u64) -> Result<T, ()> {
        todo!()
    }
}

pub struct LinearProbingRemovalTombstone {}

impl<K: PartialEq> Remove<K> for LinearProbingRemovalTombstone {
    fn default() -> Self {
        Self {}
    }

    fn remove<T>(&self, table: &mut RawHashTable, key: &K, hash: u64) -> Result<T, ()> {
        todo!()
    }
}
