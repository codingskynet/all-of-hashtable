use all_of_hashtable::{
    open_addressing::{linear_probing::LinearProbing, OpenAddressingHashTable},
    HashMap,
};

#[test]
fn test_open_addressing_hashtable() {
    let mut table = OpenAddressingHashTable::<u64, u64, LinearProbing>::new();

    for i in 0..1000 {
        assert_eq!(table.insert(i, i), Ok(()));
    }

    for i in 0..1000 {
        assert_eq!(table.lookup(&i), Some(&i));
    }

    table.print();
}
