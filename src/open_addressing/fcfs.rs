use std::{mem, ptr::NonNull};

use crate::{EntryResult, RawHashTable};

use super::EntryBucket;

pub struct FCFS;

impl FCFS {
    pub fn lookup<K, V, F>(
        table: &RawHashTable,
        key: &K,
        hash: u64,
        mut offset: F,
        tombstone: bool,
    ) -> EntryResult<EntryBucket<K, V>>
    where
        K: PartialEq,
        F: FnMut() -> usize,
    {
        let hash_index = hash as usize & table.mask;

        let first_bucket = table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>;
        let mut bucket = unsafe { first_bucket.add(hash_index) };

        let mut tombstone_ptr = None;

        loop {
            match unsafe { &*bucket } {
                EntryBucket::None => {
                    if tombstone && tombstone_ptr.is_some() {
                        return EntryResult::None(tombstone_ptr.unwrap());
                    } else {
                        return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
                EntryBucket::Tombstone => {
                    if tombstone && tombstone_ptr.is_none() {
                        tombstone_ptr = Some(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return EntryResult::Some(NonNull::new(bucket as *mut _).unwrap());
                    }
                }
            }

            let next_index = hash_index.wrapping_add(offset()) & table.mask;

            if next_index == hash_index {
                return EntryResult::Full;
            }

            unsafe { bucket = first_bucket.add(next_index) }
        }
    }

    pub fn remove<K, V, F>(
        table: &RawHashTable,
        key: &K,
        hash: u64,
        offset: F,
        tombstone: bool,
    ) -> Result<EntryBucket<K, V>, ()> 
    where
        K :PartialEq,
        F: FnMut() -> usize
    {
        if tombstone {
            match Self::lookup(table, key, hash, offset, false) {
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
