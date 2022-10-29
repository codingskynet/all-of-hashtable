use std::cell::RefCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr;
use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::BuildHasher,
};

use crate::Stat;
use crate::{Entry, InsertResult, RawHashTable};

use super::{Bucket, EntryBucket, FCFS};

pub struct FcfsDoubleHashing<T = DefaultHasher> {
    hasher: Box<dyn BuildHasher<Hasher = T>>,
    tombstone: bool,
    stat: RefCell<Stat>,
}

impl Default for FcfsDoubleHashing {
    fn default() -> Self {
        Self {
            hasher: Box::new(RandomState::new()),
            tombstone: true,
            stat: RefCell::new(Stat::default()),
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

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step = step.wrapping_add(second_hash);

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let result = FCFS::lookup(table, &bucket.key, bucket.hash, offset);

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().insert_psl.push(psl);

        if let Ok(entry_bucket) = result {
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

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step = step.wrapping_add(second_hash);

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let result = FCFS::lookup(table, key, hash, offset);

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().lookup_psl.push(psl);

        if let Ok(entry_bucket) = result {
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

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step = step.wrapping_add(second_hash);

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        debug_assert!(self.tombstone); // double hashing does not support backshift

        let entry_bucket = FCFS::remove(table, key, hash, offset, self.tombstone)?;

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().remove_psl.push(psl);

        match entry_bucket {
            EntryBucket::Some(bucket) => Ok(bucket),
            _ => Err(()),
        }
    }

    fn stat(&self) -> Stat {
        self.stat.borrow().clone()
    }
}
