use solution::*;

#[test]
fn test_classic_pangram() {
    assert!(is_pangram("The quick brown fox jumps over the lazy dog"));
}

#[test]
fn test_not_pangram() {
    assert!(!is_pangram("The quick brown fox jumps over the lazy do"));
}

#[test]
fn test_empty_string() {
    assert!(!is_pangram(""));
}

#[test]
fn test_mixed_case_pangram() {
    assert!(is_pangram("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"));
}

#[test]
fn test_with_numbers_and_symbols() {
    assert!(is_pangram("7h3 qu1ck br0wn f0x jumps 0v3r th3 l@zy d0g abcdefghijklmnopqrstuvwxyz"));
}

#[test]
fn test_alphabet_only() {
    assert!(is_pangram("abcdefghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijklmnopqrstuvwxy"));
}
