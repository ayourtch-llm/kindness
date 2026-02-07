use solution::*;

#[test]
fn test_even_positive() {
    assert_eq!(is_even(4), true);
}

#[test]
fn test_odd_positive() {
    assert_eq!(is_even(7), false);
}

#[test]
fn test_zero() {
    assert_eq!(is_even(0), true);
}

#[test]
fn test_negative_even() {
    assert_eq!(is_even(-2), true);
}

#[test]
fn test_negative_odd() {
    assert_eq!(is_even(-3), false);
}
