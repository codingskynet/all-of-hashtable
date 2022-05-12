use std::alloc::{alloc, dealloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::marker::PhantomData;
use std::{hash::Hash, ptr::NonNull};
use std::{mem, ptr};

use crate::{Entry, EntryResult, HashMap, HashTable, RawHashTable, INITIAL_SIZE, LOAD_FACTOR};

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
                ptr::write(raw.add(i), EntryBucket::None);
            }

            raw
        }
    }

    pub fn dealloc(ptr: NonNull<u8>, size: usize) {
        let layout = match Layout::array::<Self>(size) {
            Ok(layout) => layout,
            Err(_) => panic!("Cannot initialize EntryBuckets"),
        };

        unsafe {
            dealloc(ptr.as_ptr(), layout);
        }
    }
}

pub struct OpenAddressingHashTable<K, V, E, S = BuildHasherDefault<DefaultHasher>>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    S: BuildHasher,
{
    hashtable: HashTable<K, V, S, E, EntryBucket<K, V>>,
}

impl<K, V, E> OpenAddressingHashTable<K, V, E>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
{
    pub fn new() -> Self {
        Self::with_hasher(BuildHasherDefault::<DefaultHasher>::default())
    }
}

impl<K: PartialEq + Hash + Clone + Debug, V: Debug, E: Entry<K, EntryBucket<K, V>>>
    OpenAddressingHashTable<K, V, E>
{
    pub fn print(&self) {
        let size = self.hashtable.inner.mask + 1;

        println!("-------------------- OpenAddressingHashTable --------------------");
        println!("size: {}", size);

        for i in 0..size {
            print!("{:#010X}: ", i);
            let bucket = self.hashtable.inner.buckets.as_ptr() as *const EntryBucket<K, V>;

            match unsafe { &*bucket.add(i) } {
                EntryBucket::None => println!("None"),
                EntryBucket::Some(entry) => {
                    println!("{:#018X}, ({:?}, {:?})", entry.hash, entry.key, entry.value)
                }
                EntryBucket::Tombstone => println!("TOMESTONE"),
            }
        }
        println!("-----------------------------------------------------------------");
    }
}

impl<K, V, E, S> OpenAddressingHashTable<K, V, E, S>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    S: BuildHasher,
{
    pub fn new_with_properties(hasher: S, entry: E, initial_size: usize, load_factor: f32) -> Self {
        let hashtable = HashTable {
            hasher,
            inner: RawHashTable {
                buckets: NonNull::new(EntryBucket::<K, V>::alloc(initial_size) as *mut u8).unwrap(),
                mask: initial_size - 1,
            },
            count: 0,
            load_factor,
            entry: Box::new(entry),
            _marker: PhantomData,
        };

        Self { hashtable }
    }

    fn insert_bucket(&mut self, entry: Bucket<K, V>) -> Result<(), V> {
        let bucket = self
            .hashtable
            .entry
            .entry(&self.hashtable.inner, &entry.key, entry.hash);

        match bucket {
            EntryResult::None(mut ptr) => {
                unsafe { *ptr.as_mut() = EntryBucket::Some(entry) };
                self.hashtable.count += 1;
                Ok(())
            }
            EntryResult::Some(_) => Err(*entry.value),
            EntryResult::Full => {
                self.resize((self.hashtable.inner.mask + 1) << 1);
                self.insert_bucket(entry)
            }
        }
    }

    fn resize(&mut self, new_size: usize) {
        let new_inner = RawHashTable {
            buckets: NonNull::new(EntryBucket::<K, V>::alloc(new_size) as *mut u8).unwrap(),
            mask: new_size - 1,
        };

        let old_inner = mem::replace(&mut self.hashtable.inner, new_inner);

        self.hashtable.count = 0;
        for index in 0..=old_inner.mask {
            let entry_bucket = unsafe {
                ptr::read((old_inner.buckets.as_ptr() as *const EntryBucket<K, V>).add(index))
            };

            if let EntryBucket::Some(bucket) = entry_bucket {
                assert!(self.insert_bucket(bucket).is_ok());
            }
        }

        EntryBucket::<K, V>::dealloc(old_inner.buckets, old_inner.mask + 1);
    }
}

impl<K, V, E, S> HashMap<K, V, S> for OpenAddressingHashTable<K, V, E, S>
where
    K: PartialEq + Hash + Clone,
    E: Entry<K, EntryBucket<K, V>>,
    S: BuildHasher,
{
    fn with_hasher(hasher: S) -> Self {
        Self::new_with_properties(hasher, E::default(), INITIAL_SIZE, LOAD_FACTOR)
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), V> {
        let hash = self.hashtable.hasher.hash_one(key.clone());

        let bucket = Bucket {
            key: key.clone(),
            hash,
            value: Box::new(value),
        };

        if self.hashtable.count
            >= ((self.hashtable.inner.mask + 1) as f32 * self.hashtable.load_factor) as usize
        {
            self.resize((self.hashtable.inner.mask + 1) << 1);
        }

        self.insert_bucket(bucket)
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
