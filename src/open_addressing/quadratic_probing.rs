use std::{cell::RefCell, ptr};

use crate::{Entry, InsertResult, RawHashTable, Stat};

use super::{Bucket, EntryBucket, FCFS};

pub struct FcfsQuadraticProbing {
    tombstone: bool,
    stat: RefCell<Stat>,
}

impl Default for FcfsQuadraticProbing {
    fn default() -> Self {
        Self {
            tombstone: true,
            stat: RefCell::new(Stat::default()),
        }
    }
}

impl<K: PartialEq, V> Entry<K, Bucket<K, V>> for FcfsQuadraticProbing {
    fn insert(&mut self, table: &RawHashTable, bucket: Bucket<K, V>) -> InsertResult<Bucket<K, V>> {
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += 1;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step * step
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
            step += 1;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step * step
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
        let mut step = 0;

        #[cfg(feature = "stat")]
        let mut psl = 0;

        let offset = || {
            step += 1;

            #[cfg(feature = "stat")]
            {
                psl += 1;
            }

            step * step
        };

        debug_assert!(self.tombstone); // quadratic probing does not support backshift

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
