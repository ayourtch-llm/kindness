use solution::*;

#[test]
fn test_simple() {
    assert_eq!(reverse_string("hello"), "olleh");
}

#[test]
fn test_empty() {
    assert_eq!(reverse_string(""), "");
}

#[test]
fn test_single_char() {
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn test_palindrome() {
    assert_eq!(reverse_string("madam"), "madam");
}

#[test]
fn test_spaces() {
    assert_eq!(reverse_string("hello world"), "dlrow olleh");
}
