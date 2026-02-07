use solution::*;

#[test]
fn test_exact_match() {
    assert!(is_match("abc", "abc"));
    assert!(!is_match("abc", "abd"));
}

#[test]
fn test_dot_wildcard() {
    assert!(is_match("abc", "a.c"));
    assert!(is_match("axc", "a.c"));
    assert!(!is_match("ac", "a.c"));
    assert!(is_match("abc", "..."));
}

#[test]
fn test_star_repetition() {
    assert!(is_match("aa", "a*"));
    assert!(is_match("aaa", "a*"));
    assert!(is_match("", "a*"));
    assert!(is_match("b", "a*b"));
    assert!(is_match("aab", "a*b"));
}

#[test]
fn test_dot_star() {
    assert!(is_match("anything goes here", ".*"));
    assert!(is_match("", ".*"));
    assert!(is_match("abc", ".*c"));
    assert!(is_match("abc", "a.*"));
}

#[test]
fn test_complex_patterns() {
    assert!(is_match("aab", "c*a*b"));
    assert!(!is_match("mississippi", "mis*is*p*."));
    assert!(is_match("mississippi", "mis*is*s*ip*pi"));
    assert!(is_match("ab", ".*.."));
    assert!(!is_match("ab", ".*..."));
}

#[test]
fn test_empty_text_and_pattern() {
    assert!(is_match("", ""));
    assert!(!is_match("a", ""));
    assert!(is_match("", "a*b*c*"));
}

#[test]
fn test_trailing_star() {
    assert!(is_match("a", "ab*"));
    assert!(is_match("abc", "abc*"));
    assert!(is_match("abccc", "abc*"));
    assert!(!is_match("abcd", "abc*"));
}