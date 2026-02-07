use solution::*;

#[test]
fn test_positive_numbers() {
    assert_eq!(max_element(&[1, 3, 2, 5, 4]), Some(5));
}

#[test]
fn test_empty_slice() {
    assert_eq!(max_element(&[]), None);
}

#[test]
fn test_single_element() {
    assert_eq!(max_element(&[42]), Some(42));
}

#[test]
fn test_negative_numbers() {
    assert_eq!(max_element(&[-5, -1, -10, -3]), Some(-1));
}
