use solution::*;

#[test]
fn test_basic() {
    let result = top_k_frequent(&[1, 1, 1, 2, 2, 3], 2);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
}

#[test]
fn test_single_element() {
    let result = top_k_frequent(&[1], 1);
    assert_eq!(result, vec![1]);
}

#[test]
fn test_all_same_frequency() {
    let result = top_k_frequent(&[1, 2, 3, 4], 2);
    assert_eq!(result.len(), 2);
}

#[test]
fn test_k_equals_unique_count() {
    let mut result = top_k_frequent(&[3, 3, 1, 1, 2, 2], 3);
    result.sort();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_negative_numbers() {
    let result = top_k_frequent(&[-1, -1, -1, 2, 2, 3, 3, 3, 3], 1);
    assert_eq!(result, vec![3]);
}
