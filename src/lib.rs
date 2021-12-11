use std::{hash::{BuildHasher, Hash, Hasher}, marker::PhantomData, ptr::NonNull};

pub mod chaining;
pub mod open_addressing;

const START_MASK: usize = 0b111;

struct RawHashTable {
    buckets: NonNull<u8>,
    mask: usize,
}

struct HashTable<K: Hash, V, S: BuildHasher, E: Entry<K>, R: Remove<K>> {
    hasher: S,
    inner: RawHashTable,
    _marker: PhantomData<(K, V, E, R)>,
}

trait Entry<K: Hash> {
    fn entry<T>(table: &RawHashTable, key: &K, index: u64) -> NonNull<T>;
}

trait Remove<K: Hash> {
    fn remove<T>(table: &mut RawHashTable, key: &K, index: u64) -> T;
}

trait HashMap<K, V, S> {
    fn with_hasher(hasher: S) -> Self;
    fn insert(&mut self, key: K, value: V) -> Result<(), V>;
    fn lookup(&self, key: &K) -> Option<&V>;
    fn update(&mut self, key: &K, value: V) -> Result<V, V>;
    fn remove(&mut self, key: &K) -> Result<V, ()>;
}
