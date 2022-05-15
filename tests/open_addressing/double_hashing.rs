use all_of_hashtable::{
    open_addressing::{FcfsDoubleHashing, LcfsDoubleHashing, OpenAddressingHashTable},
    HashMap, INITIAL_SIZE, LOAD_FACTOR,
};
use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

use crate::util::stress_hashmap;

#[test]
fn test_crd_fcfs() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        FcfsDoubleHashing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    for i in 0..1000 {
        assert_eq!(table.insert(&i, i), Ok(()));
    }

    for i in 0..1000 {
        assert_eq!(table.lookup(&i), Some(&i));
    }

    for i in 0..1000 {
        assert_eq!(table.remove(&i), Ok(i));
    }

    for i in 0..1000 {
        assert_eq!(table.lookup(&i), None);
    }
}

#[test]
fn test_stress_fcfs() {
    let table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        FcfsDoubleHashing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    stress_hashmap(table, 100_000);
}

#[test]
fn test_crd_lcfs() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        LcfsDoubleHashing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    for i in 0..1000 {
        assert_eq!(table.insert(&i, i), Ok(()));
    }

    for i in 0..1000 {
        assert_eq!(table.lookup(&i), Some(&i));
    }

    for i in 0..1000 {
        assert_eq!(table.remove(&i), Ok(i));
    }

    for i in 0..1000 {
        assert_eq!(table.lookup(&i), None);
    }
}

#[test]
fn test_stress_lcfs() {
    let table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        LcfsDoubleHashing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    stress_hashmap(table, 100_000);
}
