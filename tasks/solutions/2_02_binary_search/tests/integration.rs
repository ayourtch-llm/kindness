use solution::*;

#[test]
fn test_found_middle() {
    assert_eq!(binary_search(&[1, 3, 5, 7, 9], 5), Some(2));
}

#[test]
fn test_found_first() {
    assert_eq!(binary_search(&[1, 3, 5, 7, 9], 1), Some(0));
}

#[test]
fn test_found_last() {
    assert_eq!(binary_search(&[1, 3, 5, 7, 9], 9), Some(4));
}

#[test]
fn test_not_found() {
    assert_eq!(binary_search(&[1, 3, 5, 7, 9], 4), None);
}

#[test]
fn test_empty_slice() {
    assert_eq!(binary_search(&[], 1), None);
}

#[test]
fn test_single_element_found() {
    assert_eq!(binary_search(&[42], 42), Some(0));
}

#[test]
fn test_single_element_not_found() {
    assert_eq!(binary_search(&[42], 7), None);
}
