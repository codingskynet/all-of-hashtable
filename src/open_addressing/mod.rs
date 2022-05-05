use std::alloc::{alloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::marker::PhantomData;
use std::{hash::Hash, ptr::NonNull};

use crate::{Entry, EntryResult, HashMap, HashTable, RawHashTable, Remove, START_MASK};

pub mod linear_probing;

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
    pub fn alloc(size: usize) -> *mut Self {
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

pub struct OpenAddressingHashTable<K, V, E, R, S = BuildHasherDefault<DefaultHasher>>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    R: Remove<K, EntryBucket<K, V>>,
    S: BuildHasher,
{
    hashtable: HashTable<K, V, S, E, R, EntryBucket<K, V>>,
}

impl<K, V, E, R> OpenAddressingHashTable<K, V, E, R>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    R: Remove<K, EntryBucket<K, V>>,
{
    pub fn new() -> Self {
        Self::with_hasher(BuildHasherDefault::<DefaultHasher>::default())
    }
}

impl<
        K: PartialEq + Hash + Clone + Debug,
        V: Debug,
        E: Entry<K, EntryBucket<K, V>>,
        R: Remove<K, EntryBucket<K, V>>,
    > OpenAddressingHashTable<K, V, E, R>
{
    pub fn print(&self) {
        let size = self.hashtable.inner.mask + 1;

        println!("-------------------- OpenAddressingHashTable --------------------");
        println!("size: {}", size);

        for i in 0..size {
            print!("{:#08X}: ", i);
            let bucket = self.hashtable.inner.buckets.as_ptr() as *const EntryBucket<K, V>;

            match unsafe { &*bucket.add(i) } {
                EntryBucket::None => println!("None"),
                EntryBucket::Some(entry) => {
                    println!("{:#16X}, ({:?}, {:?})", entry.hash, entry.key, entry.value)
                }
                EntryBucket::Tombstone => println!("TOMESTONE"),
            }
        }
        println!("-----------------------------------------------------------------");
    }
}

impl<K, V, E, R, S> OpenAddressingHashTable<K, V, E, R, S>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    R: Remove<K, EntryBucket<K, V>>,
    S: BuildHasher,
{
    pub fn new_with_properties(hasher: S, entry: E, remove: R) -> Self {
        let hashtable = HashTable {
            hasher,
            inner: RawHashTable {
                buckets: NonNull::new(EntryBucket::<K, V>::alloc(START_MASK + 1) as *mut u8)
                    .unwrap(),
                mask: START_MASK,
            },
            entry: Box::new(entry),
            remove: Box::new(remove),
            _marker: PhantomData,
        };

        Self { hashtable }
    }
}

impl<K, V, E, R, S> HashMap<K, V, S> for OpenAddressingHashTable<K, V, E, R, S>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    R: Remove<K, EntryBucket<K, V>>,
    S: BuildHasher,
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
            remove: Box::new(R::default()),
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

        let bucket = self
            .hashtable
            .entry
            .entry(&self.hashtable.inner, &key, hash);
        match bucket {
            EntryResult::None(mut ptr) => {
                unsafe { *ptr.as_mut() = EntryBucket::Some(entry) };
                Ok(())
            }
            EntryResult::Some(_) => Err(*entry.value),
            EntryResult::Full => todo!("The table is full!"),
        }
    }

    fn lookup(&self, key: &K) -> Option<&V> {
        let hash = self.hashtable.hasher.hash_one(key.clone());

        let bucket = self
            .hashtable
            .entry
            .entry(&self.hashtable.inner, &key, hash);
        match bucket {
            EntryResult::None(_) => None,
            EntryResult::Some(ptr) => unsafe {
                match ptr.as_ref() {
                    EntryBucket::None | EntryBucket::Tombstone => unreachable!(),
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
