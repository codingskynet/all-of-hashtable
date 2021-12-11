use std::alloc::alloc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::marker::PhantomData;
use std::{alloc::Layout, hash::Hash, ptr::NonNull};

use crate::{Entry, HashMap, HashTable, RawHashTable, Remove, START_MASK};

struct Bucket<K, V> {
    key: K,
    hash: u64,
    value: Box<V>,
}

enum EntryBucket<K, V> {
    None,
    Some(Bucket<K, V>),
    Tombstone,
}

impl<K, V> EntryBucket<K, V> {
    fn alloc(size: usize) -> *mut Self {
        let layout = match Layout::array::<Self>(size) {
            Ok(layout) => layout,
            Err(_) => panic!("Cannot initialize EntryBuckets"),
        };

        // allocate and init with None
        unsafe {
            let raw = alloc(layout) as *mut Self;

            for i in 0..size {
                *raw.add(i) = EntryBucket::None;
            }

            raw
        }
    }
}

struct OpenAddressingHashTable<
    K: Hash,
    V,
    E: Entry<K>,
    R: Remove<K>,
    S: BuildHasher = BuildHasherDefault<DefaultHasher>,
> {
    hashtable: HashTable<K, V, S, E, R>,
}

impl<K: Hash, V, E: Entry<K>, R: Remove<K>>
    OpenAddressingHashTable<K, V, E, R, BuildHasherDefault<DefaultHasher>>
{
    fn new() -> Self {
        Self::with_hasher(BuildHasherDefault::<DefaultHasher>::default())
    }
}

impl<K: Hash, V, E: Entry<K>, R: Remove<K>, S: BuildHasher> HashMap<K, V, S>
    for OpenAddressingHashTable<K, V, E, R, S>
{
    fn with_hasher(hasher: S) -> Self {
        let hashtable = HashTable {
            hasher,
            inner: RawHashTable {
                buckets: NonNull::new(EntryBucket::<K, V>::alloc(START_MASK + 1) as *mut u8)
                    .unwrap(),
                mask: START_MASK,
            },
            _marker: PhantomData,
        };

        Self { hashtable }
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), V> {
        todo!()
    }

    fn lookup(&self, key: &K) -> Option<&V> {
        todo!()
    }

    fn update(&mut self, key: &K, value: V) -> Result<V, V> {
        todo!()
    }

    fn remove(&mut self, key: &K) -> Result<V, ()> {
        todo!()
    }
}
