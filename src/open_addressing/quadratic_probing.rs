use std::{mem, ptr::NonNull};

use crate::{Entry, EntryResult, RawHashTable};

use super::EntryBucket;

pub struct QuadraticProbing {
    tombstone: bool,
}

impl Default for QuadraticProbing {
    fn default() -> Self {
        Self { tombstone: true }
    }
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for QuadraticProbing {
    fn lookup(
        &self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
        tombstone: bool,
    ) -> EntryResult<EntryBucket<K, V>> {
        let hash_index = hash as usize & table.mask;
        let mut offset = 0;

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

            offset += 1;
            let next_index = (hash_index + offset * offset) & table.mask;

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
                EntryResult::Some(mut ptr) => unsafe {
                    Ok(mem::replace(ptr.as_mut(), EntryBucket::Tombstone))
                },
                _ => Err(()),
            }
        } else {
            todo!("Backshift is not implemented now.")
        }
    }
}
