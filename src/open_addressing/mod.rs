use std::alloc::alloc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::marker::PhantomData;
use std::{alloc::Layout, hash::Hash, ptr::NonNull};

use crate::{Entry, EntryResult, HashMap, HashTable, RawHashTable, Remove, START_MASK};

pub struct Bucket<K, V> {
    key: K,
    hash: u64,
    value: Box<V>,
}

pub enum EntryBucket<K, V> {
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

pub struct LinearProbing<K, V> {
    step: usize,
    _marker: PhantomData<(K, V)>,
}

impl<K: PartialEq, V> Entry<K, EntryBucket<K, V>> for LinearProbing<K, V> {
    fn default() -> Self {
        LinearProbing { step : 1, _marker: PhantomData}
    }

    fn entry(&self, table: &RawHashTable, key: &K, hash: u64) -> EntryResult<EntryBucket<K, V>> {
        let mut index = hash as usize & table.mask;

        let initial_bucket = unsafe {
            (table.buckets.as_ptr() as *const u8 as *const EntryBucket<K, V>)
                .add(index)
        };

        let mut bucket = initial_bucket;

        loop {
            match unsafe { &*bucket } {
                EntryBucket::None | EntryBucket::Tombstone => {
                    return EntryResult::None(NonNull::new(bucket as *mut _).unwrap());
                }
                EntryBucket::Some(entry_bucket) => {
                    if entry_bucket.hash == hash && entry_bucket.key == *key {
                        return EntryResult::Some(NonNull::new(bucket as *mut _).unwrap());
                    }

                    index = (index + self.step) & table.mask;
                    unsafe { bucket = bucket.add(index) }
                }
            }

            if bucket == initial_bucket {
                return EntryResult::Full
            }
        }
    }
}

pub struct LinearProbingRemovalTombstone {}

impl<K: PartialEq> Remove<K> for LinearProbingRemovalTombstone {
    fn remove<T>(&self, table: &mut RawHashTable, key: &K, hash: u64) -> Result<T, ()> {
        todo!()
    }
}

pub struct OpenAddressingHashTable<
    K: PartialEq + Hash + Clone,
    V,
    E: Entry<K, EntryBucket<K, V>>,
    R: Remove<K>,
    S: BuildHasher = BuildHasherDefault<DefaultHasher>,
> {
    hashtable: HashTable<K, V, S, E, R, EntryBucket<K, V>>,
}

impl<K: PartialEq + Hash + Clone, V, E: Entry<K, EntryBucket<K, V>>, R: Remove<K>>
    OpenAddressingHashTable<K, V, E, R>
{
    pub fn new() -> Self {
        Self::with_hasher(BuildHasherDefault::<DefaultHasher>::default())
    }
}

impl<K: PartialEq + Hash + Clone, V, E: Entry<K, EntryBucket<K, V>>, R: Remove<K>, S: BuildHasher>
    OpenAddressingHashTable<K, V, E, R, S>
{
    pub fn new_with_properties(hasher: S, entry: E) -> Self {
        let hashtable = HashTable {
            hasher,
            inner: RawHashTable {
                buckets: NonNull::new(EntryBucket::<K, V>::alloc(START_MASK + 1) as *mut u8)
                    .unwrap(),
                mask: START_MASK,
            },
            entry: Box::new(entry),
            _marker: PhantomData,
        };

        Self { hashtable }
    }
}


impl<K: PartialEq + Hash + Clone, V, E: Entry<K, EntryBucket<K, V>>, R: Remove<K>, S: BuildHasher> HashMap<K, V, S>
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
            entry: Box::new(E::default()),
            _marker: PhantomData,
        };

        Self { hashtable }
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), V> {
        let hash = self.hashtable.hasher.hash_one(key.clone());

        let entry = Bucket {
            key: key.clone(),
            hash,
            value: Box::new(value),
        };


        let bucket = self.hashtable.entry.entry(&self.hashtable.inner, &key, hash);
        match bucket {
            EntryResult::None(mut ptr) => {
                unsafe { *ptr.as_mut() = EntryBucket::Some(entry) };
                Ok(())
            },
            EntryResult::Some(_) => Err(*entry.value),
            EntryResult::Full => todo!("The table is full!"),
        }
    }

    fn lookup(&self, key: &K) -> Option<&V> {
        let hash = self.hashtable.hasher.hash_one(key.clone());

        let bucket = self.hashtable.entry.entry(&self.hashtable.inner, &key, hash);
        match bucket {
            EntryResult::None(_) => None,
            EntryResult::Some(ptr) => unsafe {
                match ptr.as_ref() {
                    EntryBucket::None | EntryBucket::Tombstone=> unreachable!(),
                    EntryBucket::Some(entry) => Some(entry.value.as_ref()),
                }
            },
            EntryResult::Full => unreachable!(),
        }
    }

    fn update(&mut self, key: &K, value: V) -> Result<V, V> {
        todo!()
    }

    fn remove(&mut self, key: &K) -> Result<V, ()> {
        todo!()
    }
}
