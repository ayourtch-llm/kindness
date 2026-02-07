use solution::*;

#[test]
fn test_lowercase() {
    assert_eq!(count_vowels("hello"), 2);
}

#[test]
fn test_uppercase() {
    assert_eq!(count_vowels("AEIOU"), 5);
}

#[test]
fn test_no_vowels() {
    assert_eq!(count_vowels("rhythm"), 0);
}

#[test]
fn test_empty_string() {
    assert_eq!(count_vowels(""), 0);
}

#[test]
fn test_mixed_case() {
    assert_eq!(count_vowels("HeLLo WoRLd"), 3);
}
