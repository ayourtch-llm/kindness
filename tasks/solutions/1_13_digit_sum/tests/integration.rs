use solution::*;

#[test]
fn test_single_digit() {
    assert_eq!(digital_root(0), 0);
    assert_eq!(digital_root(5), 5);
}

#[test]
fn test_two_digits() {
    assert_eq!(digital_root(16), 7);
    assert_eq!(digital_root(99), 9);
}

#[test]
fn test_942() {
    assert_eq!(digital_root(942), 6);
}

#[test]
fn test_large_number() {
    assert_eq!(digital_root(493193), 2);
}

#[test]
fn test_all_nines() {
    assert_eq!(digital_root(999), 9);
    assert_eq!(digital_root(9999999), 9);
}
