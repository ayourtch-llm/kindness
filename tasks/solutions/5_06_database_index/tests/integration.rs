use solution::*;

#[test]
fn insert_and_get() {
    let mut tree = BTreeIndex::<i32, String>::new(3);
    tree.insert(5, "five".into());
    tree.insert(3, "three".into());
    tree.insert(7, "seven".into());
    tree.insert(1, "one".into());
    tree.insert(9, "nine".into());

    assert_eq!(tree.get(&5), Some(&"five".to_string()));
    assert_eq!(tree.get(&1), Some(&"one".to_string()));
    assert_eq!(tree.get(&9), Some(&"nine".to_string()));
    assert_eq!(tree.get(&4), None);
    assert_eq!(tree.len(), 5);
}

#[test]
fn range_query() {
    let mut tree = BTreeIndex::<i32, i32>::new(4);
    for i in 0..20 {
        tree.insert(i, i * 10);
    }
    let results = tree.range(&5, &10);
    let keys: Vec<i32> = results.iter().map(|(&k, _)| k).collect();
    assert_eq!(keys, vec![5, 6, 7, 8, 9, 10]);
    let vals: Vec<i32> = results.iter().map(|(_, &v)| v).collect();
    assert_eq!(vals, vec![50, 60, 70, 80, 90, 100]);
}

#[test]
fn remove_and_rebalance() {
    let mut tree = BTreeIndex::<i32, &str>::new(3);
    for i in 0..10 {
        tree.insert(i, "val");
    }
    assert_eq!(tree.len(), 10);

    assert_eq!(tree.remove(&5), Some("val"));
    assert_eq!(tree.get(&5), None);
    assert_eq!(tree.len(), 9);

    assert_eq!(tree.remove(&0), Some("val"));
    assert_eq!(tree.remove(&9), Some("val"));
    assert_eq!(tree.len(), 7);

    // Verify remaining elements are intact
    for i in [1, 2, 3, 4, 6, 7, 8] {
        assert!(tree.get(&i).is_some(), "key {} should exist", i);
    }
}

#[test]
fn update_existing_key() {
    let mut tree = BTreeIndex::<String, i32>::new(3);
    tree.insert("key".into(), 1);
    assert_eq!(tree.get(&"key".to_string()), Some(&1));
    tree.insert("key".into(), 2);
    assert_eq!(tree.get(&"key".to_string()), Some(&2));
    assert_eq!(tree.len(), 1);
}

#[test]
fn many_insertions_and_deletions() {
    let mut tree = BTreeIndex::<i32, i32>::new(4);
    for i in 0..100 {
        tree.insert(i, i);
    }
    assert_eq!(tree.len(), 100);

    for i in (0..100).step_by(2) {
        tree.remove(&i);
    }
    assert_eq!(tree.len(), 50);

    for i in (1..100).step_by(2) {
        assert_eq!(tree.get(&i), Some(&i));
    }
    for i in (0..100).step_by(2) {
        assert_eq!(tree.get(&i), None);
    }
}

#[test]
fn remove_nonexistent() {
    let mut tree = BTreeIndex::<i32, i32>::new(3);
    tree.insert(1, 10);
    assert_eq!(tree.remove(&999), None);
    assert_eq!(tree.len(), 1);
}
