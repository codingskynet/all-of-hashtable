use std::hash::Hash;
use std::hash::Hasher;
use std::{
    collections::hash_map::{DefaultHasher, RandomState},
    hash::BuildHasher,
};

use crate::{Entry, EntryResult, RawHashTable};

use super::{EntryBucket, FCFS};

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
        let mut step: usize = 0;
        let second_hash = self.hash_one(key) as usize;

        let offset = || {
            step = step.wrapping_add(second_hash);
            step
        };

        FCFS::lookup(table, key, hash, offset, tombstone)
    }

    fn remove(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
    ) -> Result<EntryBucket<K, V>, ()> {
        let mut step: usize = 0;
        let second_hash = self.hash_one(key) as usize;

        let offset = || {
            step = step.wrapping_add(second_hash);
            step
        };

        FCFS::remove(table, key, hash, offset, self.tombstone)
    }
}
