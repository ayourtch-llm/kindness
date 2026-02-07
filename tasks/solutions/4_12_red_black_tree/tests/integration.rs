use solution::*;

#[test]
fn test_empty() {
    let tree: RBTree<i32> = RBTree::new();
    assert_eq!(tree.len(), 0);
    assert!(!tree.contains(&5));
    assert!(tree.to_sorted_vec().is_empty());
}

#[test]
fn test_insert_and_contains() {
    let mut tree = RBTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&7));
    assert_eq!(tree.len(), 3);
}

#[test]
fn test_sorted_order() {
    let mut tree = RBTree::new();
    for &val in &[50, 25, 75, 10, 30, 60, 80] {
        tree.insert(val);
    }
    let sorted = tree.to_sorted_vec();
    assert_eq!(sorted, vec![&10, &25, &30, &50, &60, &75, &80]);
}

#[test]
fn test_sequential_insert() {
    let mut tree = RBTree::new();
    // Inserting in order would make a degenerate BST, but RB-tree should balance
    for i in 1..=20 {
        tree.insert(i);
    }
    assert_eq!(tree.len(), 20);
    let sorted = tree.to_sorted_vec();
    let expected: Vec<i32> = (1..=20).collect();
    let sorted_vals: Vec<i32> = sorted.iter().map(|&&x| x).collect();
    assert_eq!(sorted_vals, expected);
}

#[test]
fn test_duplicates() {
    let mut tree = RBTree::new();
    tree.insert(5);
    tree.insert(5);
    tree.insert(5);
    assert_eq!(tree.len(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_large_random() {
    let mut tree = RBTree::new();
    let values = vec![42, 17, 88, 3, 55, 71, 9, 33, 96, 1, 64, 27, 81, 14, 49];
    for &v in &values {
        tree.insert(v);
    }
    assert_eq!(tree.len(), 15);
    let sorted = tree.to_sorted_vec();
    let mut expected = values.clone();
    expected.sort();
    let sorted_vals: Vec<i32> = sorted.iter().map(|&&x| x).collect();
    assert_eq!(sorted_vals, expected);
}
