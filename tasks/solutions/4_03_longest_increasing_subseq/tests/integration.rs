use solution::*;

#[test]
fn test_empty() {
    assert_eq!(lis(&[]), 0);
}

#[test]
fn test_single() {
    assert_eq!(lis(&[5]), 1);
}

#[test]
fn test_sorted() {
    assert_eq!(lis(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn test_reverse_sorted() {
    assert_eq!(lis(&[5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_classic() {
    assert_eq!(lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4); // 2,3,7,18 or 2,3,7,101
}

#[test]
fn test_duplicates() {
    // Strictly increasing, so duplicates don't extend
    assert_eq!(lis(&[3, 3, 3, 3]), 1);
}

#[test]
fn test_mixed() {
    assert_eq!(lis(&[0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]), 6);
}
