use solution::*;

#[test]
fn test_simple_parens() {
    assert_eq!(is_valid("()"), true);
}

#[test]
fn test_multiple_types() {
    assert_eq!(is_valid("()[]{}"), true);
}

#[test]
fn test_nested() {
    assert_eq!(is_valid("{[()]}"), true);
}

#[test]
fn test_mismatched() {
    assert_eq!(is_valid("(]"), false);
}

#[test]
fn test_unclosed() {
    assert_eq!(is_valid("("), false);
}

#[test]
fn test_empty_string() {
    assert_eq!(is_valid(""), true);
}

#[test]
fn test_close_before_open() {
    assert_eq!(is_valid(")("), false);
}
