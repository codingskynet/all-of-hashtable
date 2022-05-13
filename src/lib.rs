#![feature(build_hasher_simple_hash_one)]

use std::{
    hash::{BuildHasher, Hash, BuildHasherDefault},
    marker::PhantomData,
    ptr::NonNull, collections::hash_map::DefaultHasher,
};

pub mod chaining;
pub mod open_addressing;

pub const INITIAL_SIZE: usize = 8;
pub const LOAD_FACTOR: f32 = 0.7;

pub struct RawHashTable {
    buckets: NonNull<u8>,
    mask: usize,
}

struct HashTable<K: Hash + PartialEq, V, S: BuildHasher, E: Entry<K, B>, B> {
    hasher: S,
    inner: RawHashTable,
    count: usize,
    load_factor: f32,
    entry: Box<E>,
    _marker: PhantomData<(K, V, E, B)>,
}

pub enum EntryResult<T> {
    None(NonNull<T>),
    Some(NonNull<T>),
    Full, // the all of available access entries are full(Some or Tombstone)
}

pub trait Entry<K: PartialEq, B> : Default {
    fn lookup(&self, table: &RawHashTable, key: &K, hash: u64, tombstone: bool) -> EntryResult<B>;
    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<B, ()>;
}

pub trait HashMap<K, V, S = BuildHasherDefault<DefaultHasher>> {
    fn new() -> Self;
    fn with_hasher(hasher: S) -> Self;
    fn insert(&mut self, key: &K, value: V) -> Result<(), V>;
    fn lookup(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Result<V, ()>;
}
