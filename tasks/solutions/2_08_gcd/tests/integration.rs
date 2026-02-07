use solution::*;

#[test]
fn test_basic() {
    assert_eq!(gcd(12, 8), 4);
}

#[test]
fn test_coprime() {
    assert_eq!(gcd(13, 7), 1);
}

#[test]
fn test_same_number() {
    assert_eq!(gcd(42, 42), 42);
}

#[test]
fn test_one_zero() {
    assert_eq!(gcd(0, 5), 5);
}

#[test]
fn test_both_zero() {
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn test_large_numbers() {
    assert_eq!(gcd(1_000_000_007, 999_999_937), 1);
}
