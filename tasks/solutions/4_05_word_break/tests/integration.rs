use solution::*;

#[test]
fn test_empty_string() {
    assert_eq!(word_break("", &["a", "b"]), true);
}

#[test]
fn test_single_word() {
    assert_eq!(word_break("hello", &["hello"]), true);
}

#[test]
fn test_two_words() {
    assert_eq!(word_break("leetcode", &["leet", "code"]), true);
}

#[test]
fn test_impossible() {
    assert_eq!(word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]), false);
}

#[test]
fn test_reuse_words() {
    assert_eq!(word_break("applepenapple", &["apple", "pen"]), true);
}

#[test]
fn test_prefix_trap() {
    // "aaaaaaa" with dict ["aaa","aaaa"] => 3+4=7 yes
    assert_eq!(word_break("aaaaaaa", &["aaa", "aaaa"]), true);
}

#[test]
fn test_no_match() {
    assert_eq!(word_break("xyz", &["a", "b", "c"]), false);
}
