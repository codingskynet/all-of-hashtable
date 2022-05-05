use all_of_hashtable::{
    open_addressing::{
        linear_probing::{LinearProbing, LinearProbingRemovalTombstone},
        OpenAddressingHashTable,
    },
    HashMap,
};

#[test]
fn test_open_addressing_hashtable() {
    let mut table = OpenAddressingHashTable::<
        String,
        u64,
        LinearProbing,
        LinearProbingRemovalTombstone,
    >::new();
    assert_eq!(table.insert("test".to_string(), 123), Ok(()));
    assert_eq!(table.lookup(&"test".to_string()), Some(&123));

    table.print();
}
