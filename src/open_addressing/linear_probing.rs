use std::{cell::RefCell, ptr};

use crate::{Entry, InsertResult, RawHashTable, Stat};

use super::{Bucket, EntryBucket, FCFS, LCFS};

pub struct FcfsLinearProbing {
    step: usize,
    tombstone: bool,
    stat: RefCell<Stat>,
}

impl Default for FcfsLinearProbing {
    fn default() -> Self {
        Self {
            step: 1,
            tombstone: true,
            stat: RefCell::new(Stat::default()),
        }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for FcfsLinearProbing {
    fn insert(&mut self, table: &RawHashTable, bucket: Bucket<K, V>) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

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
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let result = FCFS::lookup(table, key, hash, offset);

        if let Ok(entry_bucket) = result {
            #[cfg(feature = "stat")]
            self.stat.borrow_mut().lookup_psl.push(psl);

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

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

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

pub struct LcfsLinearProbing {
    step: usize,
    tombstone: bool,
    stat: RefCell<Stat>,
}

impl Default for LcfsLinearProbing {
    fn default() -> Self {
        Self {
            step: 1,
            tombstone: true,
            stat: RefCell::new(Stat::default()),
        }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for LcfsLinearProbing {
    fn insert(&mut self, table: &RawHashTable, bucket: Bucket<K, V>) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let result = LCFS::insert(table, offset, bucket, self.tombstone);

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().insert_psl.push(psl);

        result
    }

    fn lookup<'a>(&self, table: &'a RawHashTable, key: &K, hash: u64) -> Option<&'a Bucket<K, V>> {
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let entry_bucket = LCFS::lookup(table, key, hash, offset)?;

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().lookup_psl.push(psl);

        if let EntryBucket::Some(bucket) = entry_bucket {
            Some(&*bucket)
        } else {
            unreachable!()
        }
    }

    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<Bucket<K, V>, ()> {
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += self.step;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step
        };

        let entry_bucket = LCFS::remove(table, key, hash, offset, self.tombstone)?;

        #[cfg(feature = "stat")]
        self.stat.borrow_mut().remove_psl.push(psl);

        if let EntryBucket::Some(bucket) = entry_bucket {
            Ok(bucket)
        } else {
            unreachable!()
        }
    }

    fn stat(&self) -> Stat {
        self.stat.borrow().clone()
    }
}
