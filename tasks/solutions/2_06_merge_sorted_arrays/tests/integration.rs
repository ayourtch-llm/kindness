use solution::*;

#[test]
fn test_basic_merge() {
    assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_first_empty() {
    assert_eq!(merge_sorted(&[], &[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_second_empty() {
    assert_eq!(merge_sorted(&[4, 5, 6], &[]), vec![4, 5, 6]);
}

#[test]
fn test_both_empty() {
    assert_eq!(merge_sorted(&[], &[]), vec![]);
}

#[test]
fn test_duplicates() {
    assert_eq!(merge_sorted(&[1, 2, 2], &[2, 3, 3]), vec![1, 2, 2, 2, 3, 3]);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(merge_sorted(&[-5, -1, 0], &[-3, 2, 4]), vec![-5, -3, -1, 0, 2, 4]);
}
