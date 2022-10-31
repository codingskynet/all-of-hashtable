use all_of_hashtable::{
    open_addressing::{Bucket, FcfsLinearProbing, LcfsLinearProbing, OpenAddressingHashTable},
    HashMap, INITIAL_SIZE, LOAD_FACTOR,
};
use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

use crate::util::{draw_stat, stress_hashmap};

#[test]
fn test_crd_fcfs() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        FcfsLinearProbing::default(),
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
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        FcfsLinearProbing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    stress_hashmap(&mut table, 100_000);

    #[cfg(feature = "stat")]
    draw_stat(
        <FcfsLinearProbing as all_of_hashtable::Entry<u64, Bucket<u64, u64>>>::stat(table.entry()),
        "output/FcfsLinearProbing.png",
    );
}

#[test]
fn test_crd_lcfs() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        LcfsLinearProbing::default(),
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
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        LcfsLinearProbing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    stress_hashmap(&mut table, 100_000);

    #[cfg(feature = "stat")]
    draw_stat(
        <LcfsLinearProbing as all_of_hashtable::Entry<u64, Bucket<u64, u64>>>::stat(table.entry()),
        "output/LcfsLinearProbing.png",
    );
}
