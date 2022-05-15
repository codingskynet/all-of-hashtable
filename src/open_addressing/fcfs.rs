use std::mem;

use crate::RawHashTable;

use super::EntryBucket;

pub struct FCFS;

impl FCFS {
    pub fn lookup<'a, K, V, F>(
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
        mut offset: F,
        tombstone: bool,
    ) -> Result<&'a mut EntryBucket<K, V>, ()>
    where
        K: PartialEq,
        F: FnMut() -> usize,
    {
        let hash_index = hash as usize & table.mask;

        let first_bucket = table.buckets.as_ptr() as *mut u8 as *mut EntryBucket<K, V>;
        let mut bucket = unsafe { &mut *first_bucket.add(hash_index) };

        let mut tombstone_ptr = None;

        loop {
            match bucket {
                EntryBucket::None => {
                    return if tombstone_ptr.is_some() {
                        Ok(tombstone_ptr.unwrap())
                    } else {
                        Ok(bucket)
                    };
                }
                EntryBucket::Tombstone => {
                    debug_assert!(tombstone);

                    if tombstone_ptr.is_none() {
                        tombstone_ptr = Some(bucket);
                    }
                }
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return Ok(bucket);
                    }
                }
            }

            let next_index = hash_index.wrapping_add(offset()) & table.mask;

            if next_index == hash_index {
                return Err(());
            }

            unsafe { bucket = &mut *first_bucket.add(next_index) }
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
        K: PartialEq,
        F: FnMut() -> usize,
    {
        if tombstone {
            if let Ok(entry_bucket) = Self::lookup(table, key, hash, offset, false) {
                match entry_bucket {
                    EntryBucket::None => Err(()),
                    EntryBucket::Some(_) => Ok(mem::replace(entry_bucket, EntryBucket::Tombstone)),
                    EntryBucket::Tombstone => panic!(),
                }
            } else {
                Err(())
            }
        } else {
            todo!("Backshift is not implemented now.")
        }
    }
}
