use solution::*;

#[test]
fn test_positive_numbers() {
    assert_eq!(sum_array(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_empty_array() {
    assert_eq!(sum_array(&[]), 0);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(sum_array(&[-1, -2, -3]), -6);
}

#[test]
fn test_mixed_numbers() {
    assert_eq!(sum_array(&[-10, 5, 5]), 0);
}
