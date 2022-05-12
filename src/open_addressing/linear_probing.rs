use std::mem;
use std::ptr::NonNull;

use crate::Entry;
use crate::EntryResult;
use crate::RawHashTable;

use super::EntryBucket;

pub struct LinearProbing {
    step: usize,
    tombstone: bool,
}

impl Default for LinearProbing {
    fn default() -> Self {
        Self {
            step: 1,
            tombstone: true,
        }
    }
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for LinearProbing {
    fn lookup(&self, table: &RawHashTable, key: &K, hash: u64) -> EntryResult<EntryBucket<K, V>> {
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

    fn remove(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
    ) -> Result<EntryBucket<K, V>, ()> {
        if self.tombstone {
            match self.lookup(table, key, hash) {
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
