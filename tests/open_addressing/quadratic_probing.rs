use all_of_hashtable::{
    open_addressing::{Bucket, FcfsQuadraticProbing, OpenAddressingHashTable},
    HashMap, INITIAL_SIZE, LOAD_FACTOR,
};
use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

use crate::util::{draw_stat, stress_hashmap};

#[test]
fn test_crd_fcfs() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        FcfsQuadraticProbing::default(),
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
        FcfsQuadraticProbing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    stress_hashmap(&mut table, 100_000, false);

    #[cfg(feature = "stat")]
    {
        let stat = <FcfsQuadraticProbing as all_of_hashtable::Entry<u64, Bucket<u64, u64>>>::stat(
            table.entry(),
        );
        println!();
        stat.print();
        draw_stat(stat, "output/FcfsQuadraticProbing.png");
    }
}
