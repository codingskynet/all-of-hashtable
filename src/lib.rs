use std::{
    collections::hash_map::DefaultHasher,
    hash::{BuildHasher, BuildHasherDefault, Hash},
    marker::PhantomData,
    ptr::NonNull,
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

#[derive(Default, Clone)]
pub struct Stat {
    pub insert_psl: Vec<u8>,
    pub lookup_psl: Vec<u8>,
    pub remove_psl: Vec<u8>,
}

impl Stat {
    pub fn print(&self) {
        println!("- insert");
        println!(
            "total: {}, avg: {}",
            self.insert_psl.len(),
            self.insert_psl.iter().map(|x| *x as usize).sum::<usize>() as f64
                / self.insert_psl.len() as f64
        );

        println!("- lookup");
        println!(
            "total: {}, avg: {}",
            self.lookup_psl.len(),
            self.lookup_psl.iter().map(|x| *x as usize).sum::<usize>() as f64
                / self.lookup_psl.len() as f64
        );

        println!("- remove");
        println!(
            "total: {}, avg: {}",
            self.remove_psl.len(),
            self.remove_psl.iter().map(|x| *x as usize).sum::<usize>() as f64
                / self.remove_psl.len() as f64
        );
    }
}

pub trait Entry<K: PartialEq, B>: Default {
    fn insert(&mut self, table: &RawHashTable, bucket: B) -> InsertResult<B>;
    fn lookup<'a>(&self, table: &'a RawHashTable, key: &K, hash: u64) -> Option<&'a B>;
    fn remove(&mut self, table: &RawHashTable, key: &K, hash: u64) -> Result<B, ()>;
    fn stat(&self) -> Stat;
}

pub trait HashMap<K, V, S = BuildHasherDefault<DefaultHasher>> {
    fn new() -> Self;
    fn with_hasher(hasher: S) -> Self;
    fn insert(&mut self, key: &K, value: V) -> Result<(), V>;
    fn lookup(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Result<V, ()>;
}
