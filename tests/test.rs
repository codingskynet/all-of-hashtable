use all_of_hashtable::{open_addressing::{OpenAddressingHashTable, LinearProbing, LinearProbingRemovalTombstone}, HashMap};

#[test]
fn test_open_addressing_hashtable() {
    let mut table = OpenAddressingHashTable::<String, u64, LinearProbing<_, _>, LinearProbingRemovalTombstone>::new();
    assert_eq!(table.insert("test".to_string(), 123), Ok(()));
    assert_eq!(table.lookup(&"test".to_string()), Some(&123));
}
