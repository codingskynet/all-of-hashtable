use std::ptr::NonNull;

use crate::Entry;
use crate::EntryResult;
use crate::RawHashTable;

use super::EntryBucket;

pub struct LinearProbing {
    step: usize,
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for LinearProbing {
    fn default() -> Self {
        LinearProbing { step: 1 }
    }

    fn entry(&self, table: &RawHashTable, key: &K, hash: u64) -> EntryResult<EntryBucket<K, V>> {
        let hash_index = hash as usize & table.mask;
        let mut index = hash_index;

        let first_bucket = table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>;
        let mut bucket = unsafe { first_bucket.add(index) };

        loop {
            match unsafe { &*bucket } {
                EntryBucket::None => {
                    return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                }
                EntryBucket::Tombstone => {} // just skip this bucket
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return EntryResult::Some(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
            }

            index = (index + self.step) & table.mask;
            unsafe { bucket = first_bucket.add(index) }

            if index == hash_index {
                return EntryResult::Full;
            }
        }
    }
}
