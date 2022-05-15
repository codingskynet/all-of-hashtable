use std::ptr;

use crate::{Entry, EntryResult, InsertResult, RawHashTable};

use super::{Bucket, EntryBucket, FCFS, LCFS};

pub struct FcfsLinearProbing {
    step: usize,
    tombstone: bool,
}

impl Default for FcfsLinearProbing {
    fn default() -> Self {
        Self {
            step: 1,
            tombstone: true,
        }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for FcfsLinearProbing {
    fn insert(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
        bucket: Bucket<K, V>,
    ) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        if let Ok(entry_bucket) = FCFS::lookup(table, key, hash, offset, true) {
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

    fn lookup<'a>(
        &self,
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
        tombstone: bool,
    ) -> Option<&'a Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        if let Ok(entry_bucket) = FCFS::lookup(table, key, hash, offset, tombstone) {
            match entry_bucket {
                EntryBucket::None => None,
                EntryBucket::Some(bucket) => Some(bucket),
                EntryBucket::Tombstone => panic!(),
            }
        } else {
            None
        }
    }

    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<Bucket<K, V>, ()> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        let entry_bucket = FCFS::remove(table, key, hash, offset, self.tombstone)?;

        match entry_bucket {
            EntryBucket::Some(bucket) => Ok(bucket),
            _ => Err(()),
        }
    }
}

pub struct LcfsLinearProbing {
    step: usize,
    tombstone: bool,
}

impl Default for LcfsLinearProbing {
    fn default() -> Self {
        Self {
            step: 1,
            tombstone: true,
        }
    }
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for LcfsLinearProbing {
    fn insert(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
        bucket: EntryBucket<K, V>,
    ) -> InsertResult<EntryBucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        LCFS::insert(table, key, hash, offset, bucket, self.tombstone)
    }

    fn lookup<'a>(
        &self,
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
        tombstone: bool,
    ) -> Option<&'a EntryBucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        let bucket = LCFS::lookup(table, key, hash, offset)?;
        Some(&*bucket)
    }

    fn remove(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
    ) -> Result<EntryBucket<K, V>, ()> {
        let mut step = 0;

        let offset = || {
            step += self.step;
            step
        };

        LCFS::remove(table, key, hash, offset, self.tombstone)
    }
}
