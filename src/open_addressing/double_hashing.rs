use std::hash::Hash;
use std::hash::Hasher;
use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::BuildHasher,
    mem,
    ptr::NonNull,
};

use crate::{Entry, EntryResult, RawHashTable};

use super::EntryBucket;

pub struct DoubleHashing<T = DefaultHasher> {
    hasher: Box<dyn BuildHasher<Hasher = T>>,
    tombstone: bool,
}

impl Default for DoubleHashing {
    fn default() -> Self {
        Self {
            hasher: Box::new(RandomState::new()),
            tombstone: true,
        }
    }
}

impl DoubleHashing {
    fn hash_one<K: Hash>(&self, x: K) -> u64 {
        let mut hasher = self.hasher.build_hasher();
        x.hash(&mut hasher);
        hasher.finish()
    }
}

impl<K: PartialEq + Hash + Clone, V> Entry<K, EntryBucket<K, V>> for DoubleHashing {
    fn lookup(
        &self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
        tombstone: bool,
    ) -> EntryResult<EntryBucket<K, V>> {
        let hash_index = hash as usize & table.mask;
        let step = self.hash_one(key.clone()) as usize;

        let mut offset: usize = 0;

        let first_bucket = table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>;
        let mut bucket = unsafe { first_bucket.add(hash_index) };

        loop {
            match unsafe { &*bucket } {
                EntryBucket::None => {
                    return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                }
                EntryBucket::Tombstone => {
                    if tombstone {
                        return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return EntryResult::Some(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
            }

            offset = offset.wrapping_add(step);
            let next_index = (hash_index + offset) & table.mask;

            if next_index == hash_index {
                return EntryResult::Full;
            }

            unsafe { bucket = first_bucket.add(next_index) }
        }
    }

    fn remove(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
    ) -> Result<EntryBucket<K, V>, ()> {
        if self.tombstone {
            match self.lookup(table, key, hash, false) {
                EntryResult::Some(mut ptr, ) => unsafe {
                    Ok(mem::replace(ptr.as_mut(), EntryBucket::Tombstone))
                },
                _ => Err(()),
            }
        } else {
            todo!("Backshift is not implemented now.")
        }
    }
}
