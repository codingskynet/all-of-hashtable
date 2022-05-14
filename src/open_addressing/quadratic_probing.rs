use crate::{Entry, EntryResult, RawHashTable};

use super::{EntryBucket, FCFS};

pub struct QuadraticProbing {
    tombstone: bool,
}

impl Default for QuadraticProbing {
    fn default() -> Self {
        Self { tombstone: true }
    }
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for QuadraticProbing {
    fn lookup(
        &self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
        tombstone: bool,
    ) -> EntryResult<EntryBucket<K, V>> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        FCFS::lookup(table, key, hash, offset, tombstone)
    }

    fn remove(
        &mut self,
        table: &RawHashTable,
        key: &K,
        hash: u64,
    ) -> Result<EntryBucket<K, V>, ()> {
        let mut step = 0;

        let offset = || {
            step += 1;
            step * step
        };

        FCFS::remove(table, key, hash, offset, self.tombstone)
    }
}
