use solution::*;

#[test]
fn test_basic_anagram() {
    assert_eq!(is_anagram("listen", "silent"), true);
}

#[test]
fn test_not_anagram() {
    assert_eq!(is_anagram("hello", "world"), false);
}

#[test]
fn test_different_lengths() {
    assert_eq!(is_anagram("abc", "ab"), false);
}

#[test]
fn test_case_insensitive() {
    assert_eq!(is_anagram("Astronomer", "Moon starer"), true);
}

#[test]
fn test_empty_strings() {
    assert_eq!(is_anagram("", ""), true);
}

#[test]
fn test_with_spaces_and_punctuation() {
    assert_eq!(is_anagram("A gentleman", "Elegant man"), true);
}
