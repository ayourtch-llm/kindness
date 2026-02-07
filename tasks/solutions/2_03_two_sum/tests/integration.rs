use solution::*;

#[test]
fn test_basic() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn test_middle_elements() {
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
}

#[test]
fn test_no_solution() {
    assert_eq!(two_sum(&[1, 2, 3], 100), None);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(two_sum(&[-1, -2, -3, -4, -5], -8), Some((2, 4)));
}

#[test]
fn test_empty_slice() {
    assert_eq!(two_sum(&[], 0), None);
}

#[test]
fn test_single_element() {
    assert_eq!(two_sum(&[5], 5), None);
}
