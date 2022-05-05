#![feature(build_hasher_simple_hash_one)]

use std::{hash::{BuildHasher, Hash}, marker::PhantomData, ptr::NonNull};

pub mod chaining;
pub mod open_addressing;

const START_MASK: usize = 0b111;

pub struct RawHashTable {
    buckets: NonNull<u8>,
    mask: usize,
}

struct HashTable<K: Hash + PartialEq, V, S: BuildHasher, E: Entry<K, B>, R: Remove<K>, B> {
    hasher: S,
    inner: RawHashTable,
    entry: Box<E>,
    _marker: PhantomData<(K, V, E, R, B)>,
}

pub enum EntryResult<T> {
    None(NonNull<T>),
    Some(NonNull<T>),
    Full, // the all of available access entries are full.
}

pub trait Entry<K: PartialEq, B> {
    fn default() -> Self;
    fn entry(&self, table: &RawHashTable, key: &K, hash: u64) -> EntryResult<B>;
}

pub trait Remove<K: PartialEq> {
    fn remove<T>(&self, table: &mut RawHashTable, key: &K, hash: u64) -> Result<T, ()>;
}

pub trait HashMap<K, V, S> {
    fn with_hasher(hasher: S) -> Self;
    fn insert(&mut self, key: K, value: V) -> Result<(), V>;
    fn lookup(&self, key: &K) -> Option<&V>;
    fn update(&mut self, key: &K, value: V) -> Result<V, V>;
    fn remove(&mut self, key: &K) -> Result<V, ()>;
}
