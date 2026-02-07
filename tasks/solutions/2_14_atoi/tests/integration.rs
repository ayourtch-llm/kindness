use solution::*;

#[test]
fn test_basic_positive() {
    assert_eq!(my_atoi("42"), 42);
}

#[test]
fn test_leading_whitespace() {
    assert_eq!(my_atoi("   -42"), -42);
}

#[test]
fn test_trailing_non_digits() {
    assert_eq!(my_atoi("4193 with words"), 4193);
}

#[test]
fn test_no_digits() {
    assert_eq!(my_atoi("words and 987"), 0);
}

#[test]
fn test_overflow_positive() {
    assert_eq!(my_atoi("91283472332"), i32::MAX);
}

#[test]
fn test_overflow_negative() {
    assert_eq!(my_atoi("-91283472332"), i32::MIN);
}

#[test]
fn test_explicit_plus() {
    assert_eq!(my_atoi("+1"), 1);
}

#[test]
fn test_empty_string() {
    assert_eq!(my_atoi(""), 0);
}

#[test]
fn test_only_whitespace() {
    assert_eq!(my_atoi("   "), 0);
}

#[test]
fn test_zero() {
    assert_eq!(my_atoi("0"), 0);
}
