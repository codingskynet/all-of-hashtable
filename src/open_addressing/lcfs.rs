use std::{
    mem,
    ptr::{self, NonNull},
};

use crate::{InsertResult, RawHashTable};

use super::{Bucket, EntryBucket};

pub struct LCFS;

impl LCFS {
    // stack up for insert and remove
    fn lookup_stack<K, V, F>(
        table: &RawHashTable,
        key: &K,
        hash: u64,
        mut offset: F,
        tombstone: bool,
    ) -> Result<Vec<NonNull<EntryBucket<K, V>>>, ()>
    where
        K: PartialEq,
        F: FnMut() -> usize,
    {
        let hash_index = hash as usize & table.mask;

        let first_bucket = table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>;
        let mut bucket = unsafe { first_bucket.add(hash_index) };
        let mut buckets = Vec::new();

        let mut tombstone_buckets = None;

        loop {
            buckets.push(NonNull::new(bucket as *mut _).unwrap());

            match unsafe { &*bucket } {
                EntryBucket::None => {
                    return if let Some(result) = tombstone_buckets {
                        Ok(result)
                    } else {
                        Ok(buckets)
                    };
                }
                EntryBucket::Tombstone => {
                    debug_assert!(tombstone);

                    if tombstone_buckets.is_none() {
                        tombstone_buckets = Some(buckets.clone());
                    }
                }
                EntryBucket::Some(entry_bucket) => {
                    // todo implement for backshift on removal
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return Ok(buckets);
                    }
                }
            }

            let next_index = hash_index.wrapping_add(offset()) & table.mask;

            if next_index == hash_index {
                // insert: the table is full!
                // remove: there does not exist key!
                return Err(());
            }

            unsafe { bucket = first_bucket.add(next_index) }
        }
    }

    pub fn insert<K, V, F>(
        table: &RawHashTable,
        offset: F,
        bucket: Bucket<K, V>,
        tombstone: bool,
    ) -> InsertResult<Bucket<K, V>>
    where
        K: PartialEq,
        F: FnMut() -> usize,
    {
        let mut buckets = if let Ok(buckets) =
            LCFS::lookup_stack(table, &bucket.key, bucket.hash, offset, tombstone)
        {
            buckets
        } else {
            return InsertResult::Full(bucket);
        };

        let mut last_bucket = buckets.pop().unwrap();

        if let EntryBucket::Some(_) = unsafe { last_bucket.as_ref() } {
            return InsertResult::AlreadyExist(bucket);
        }

        while let Some(b) = buckets.pop() {
            unsafe {
                ptr::write(last_bucket.as_mut(), ptr::read(b.as_ref()));
            }

            last_bucket = b;
        }

        unsafe {
            ptr::write(last_bucket.as_mut(), EntryBucket::Some(bucket));
        }

        InsertResult::Success
    }

    pub fn lookup<'a, K, V, F>(
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
        mut offset: F,
    ) -> Option<&'a mut EntryBucket<K, V>>
    where
        K: PartialEq,
        F: FnMut() -> usize,
    {
        let hash_index = hash as usize & table.mask;

        let first_bucket = table.buckets.as_ptr() as *mut u8 as *mut EntryBucket<K, V>;
        let mut bucket = unsafe { &mut *first_bucket.add(hash_index) };

        println!("{:#018X}", hash_index);

        loop {
            println!("aaaa");
            match bucket {
                EntryBucket::None => {
                    println!("None!");
                    return None;
                }
                EntryBucket::Tombstone => {}
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return Some(bucket);
                    }
                }
            }

            let next_index = hash_index.wrapping_add(offset()) & table.mask;

            if next_index == hash_index {
                println!("Full!");
                return None;
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
            if let Some(entry_bucket) = Self::lookup(table, key, hash, offset) {
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
