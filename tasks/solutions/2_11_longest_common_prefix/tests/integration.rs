use solution::*;

#[test]
fn test_basic() {
    assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
}

#[test]
fn test_no_common_prefix() {
    assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
}

#[test]
fn test_empty_input() {
    assert_eq!(longest_common_prefix(&[]), "");
}

#[test]
fn test_single_string() {
    assert_eq!(longest_common_prefix(&["alone"]), "alone");
}

#[test]
fn test_all_identical() {
    assert_eq!(longest_common_prefix(&["same", "same", "same"]), "same");
}

#[test]
fn test_empty_string_in_input() {
    assert_eq!(longest_common_prefix(&["abc", "", "abc"]), "");
}

#[test]
fn test_one_char_prefix() {
    assert_eq!(longest_common_prefix(&["apple", "ape", "april"]), "ap");
}
