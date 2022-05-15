use std::ptr;

use crate::{Entry, InsertResult, RawHashTable};

use super::{Bucket, EntryBucket, FCFS, LCFS};

pub struct FcfsQuadraticProbing {
    tombstone: bool,
}

impl Default for FcfsQuadraticProbing {
    fn default() -> Self {
        Self { tombstone: true }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for FcfsQuadraticProbing {
    fn insert(
        &mut self,
        table: &RawHashTable,
        bucket: Bucket<K, V>,
    ) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
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

    fn lookup<'a>(
        &self,
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
    ) -> Option<&'a Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
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
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        let entry_bucket = FCFS::remove(table, key, hash, offset, self.tombstone)?;

        match entry_bucket {
            EntryBucket::Some(bucket) => Ok(bucket),
            _ => Err(()),
        }
    }
}

pub struct LcfsQuadraticProbing {
    tombstone: bool,
}

impl Default for LcfsQuadraticProbing {
    fn default() -> Self {
        Self {
            tombstone: true,
        }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for LcfsQuadraticProbing {
    fn insert(
        &mut self,
        table: &RawHashTable,
        bucket: Bucket<K, V>,
    ) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        LCFS::insert(table, offset, bucket, self.tombstone)
    }

    fn lookup<'a>(
        &self,
        table: &'a RawHashTable,
        key: &K,
        hash: u64,
    ) -> Option<&'a Bucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        let entry_bucket = LCFS::lookup(table, key, hash, offset)?;

        if let EntryBucket::Some(bucket) = entry_bucket {
            Some(&*bucket)
        } else {
            unreachable!()
        }
    }

    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<Bucket<K, V>, ()> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        let entry_bucket = LCFS::remove(table, key, hash, offset, self.tombstone)?;

        if let EntryBucket::Some(bucket) = entry_bucket {
            Ok(bucket)
        } else {
            unreachable!()
        }
    }
}
