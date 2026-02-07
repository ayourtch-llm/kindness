use solution::*;

#[test]
fn test_simple_palindrome() {
    assert_eq!(is_palindrome("racecar"), true);
}

#[test]
fn test_not_palindrome() {
    assert_eq!(is_palindrome("hello"), false);
}

#[test]
fn test_empty_string() {
    assert_eq!(is_palindrome(""), true);
}

#[test]
fn test_single_char() {
    assert_eq!(is_palindrome("a"), true);
}

#[test]
fn test_case_sensitive() {
    assert_eq!(is_palindrome("Aba"), false);
}
