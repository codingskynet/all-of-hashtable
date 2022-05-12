use std::{hash::BuildHasherDefault, collections::hash_map::DefaultHasher};

use all_of_hashtable::{open_addressing::{OpenAddressingHashTable, QuadraticProbing}, HashMap, INITIAL_SIZE, LOAD_FACTOR};

#[test]
fn test_crd() {
    let mut table = OpenAddressingHashTable::<u64, u64, _>::new_with_properties(
        BuildHasherDefault::<DefaultHasher>::default(),
        QuadraticProbing::default(),
        INITIAL_SIZE,
        LOAD_FACTOR,
    );

    for i in 0..1000 {
        assert_eq!(table.insert(i, i), Ok(()));
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
