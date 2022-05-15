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

pub enum InsertResult<T> {
    Success,
    // fail types has not inserted bucket
    AlreadyExist(T),
    Full(T),
}

pub trait Entry<K: PartialEq, B> : Default {
    fn insert(&mut self, table: &RawHashTable, key: &K, hash: u64, bucket: B) -> InsertResult<B>;
    fn lookup<'a>(&self, table: &'a RawHashTable, key: &K, hash: u64, tombstone: bool) -> Option<&'a B>;
    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<B, ()>;
}

pub trait HashMap<K, V, S = BuildHasherDefault<DefaultHasher>> {
    fn new() -> Self;
    fn with_hasher(hasher: S) -> Self;
    fn insert(&mut self, key: &K, value: V) -> Result<(), V>;
    fn lookup(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Result<V, ()>;
}
