use solution::*;

#[test]
fn test_basic() {
    assert_eq!(to_title_case("hello world"), "Hello World");
}

#[test]
fn test_mixed_case() {
    assert_eq!(to_title_case("hELLO wORLD"), "Hello World");
}

#[test]
fn test_single_word() {
    assert_eq!(to_title_case("rust"), "Rust");
}

#[test]
fn test_empty_string() {
    assert_eq!(to_title_case(""), "");
}

#[test]
fn test_already_title_case() {
    assert_eq!(to_title_case("Already Title Case"), "Already Title Case");
}

#[test]
fn test_all_uppercase() {
    assert_eq!(to_title_case("ALL UPPERCASE WORDS"), "All Uppercase Words");
}
