use std::hash::Hash;
use std::hash::Hasher;
use std::ptr;
use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::BuildHasher,
};

use crate::{Entry, InsertResult, RawHashTable};

use super::{Bucket, EntryBucket, FCFS};

pub struct FcfsDoubleHashing<T = DefaultHasher> {
    hasher: Box<dyn BuildHasher<Hasher = T>>,
    tombstone: bool,
}

impl Default for FcfsDoubleHashing {
    fn default() -> Self {
        Self {
            hasher: Box::new(RandomState::new()),
            tombstone: true,
        }
    }
}

impl FcfsDoubleHashing {
    fn hash_one<K: Hash>(&self, x: K) -> u64 {
        let mut hasher = self.hasher.build_hasher();
        x.hash(&mut hasher);
        hasher.finish()
    }
}

impl<K: PartialEq + Hash, V> Entry<K, Bucket<K, V>> for FcfsDoubleHashing {
    fn insert(&mut self, table: &RawHashTable, bucket: Bucket<K, V>) -> InsertResult<Bucket<K, V>> {
        let mut step: usize = 0;
        let second_hash = self.hash_one(&bucket.key) as usize;

        let offset = || {
            step = step.wrapping_add(second_hash);
            step
        };

        if let Ok(entry_bucket) = FCFS::lookup(table, &bucket.key, bucket.hash, offset) {
            match entry_bucket {
                EntryBucket::Some(_) => InsertResult::AlreadyExist(bucket),
                EntryBucket::None | EntryBucket::Tombstone => {
                    unsafe { ptr::write(entry_bucket, EntryBucket::Some(bucket)) };
                    InsertResult::Success
                }
            }
        } else {
            InsertResult::Full(bucket)
        }
    }

    fn lookup<'a>(&self, table: &'a RawHashTable, key: &K, hash: u64) -> Option<&'a Bucket<K, V>> {
        let mut step: usize = 0;
        let second_hash = self.hash_one(key) as usize;

        let offset = || {
            step = step.wrapping_add(second_hash);
            step
        };

        if let Ok(entry_bucket) = FCFS::lookup(table, key, hash, offset) {
            match entry_bucket {
                EntryBucket::None => None,
                EntryBucket::Some(bucket) => Some(bucket),
                EntryBucket::Tombstone => None,
            }
        } else {
            None
        }
    }

    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<Bucket<K, V>, ()> {
        let mut step: usize = 0;
        let second_hash = self.hash_one(key) as usize;

        let offset = || {
            step = step.wrapping_add(second_hash);
            step
        };

        let entry_bucket = FCFS::remove(table, key, hash, offset, self.tombstone)?;

        match entry_bucket {
            EntryBucket::Some(bucket) => Ok(bucket),
            _ => Err(()),
        }
    }
}
