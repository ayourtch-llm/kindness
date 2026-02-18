use solution::*;

#[test]
fn literal_match() {
    assert!(regex_match("hello", "hello"));
    assert!(!regex_match("hello", "world"));
    assert!(!regex_match("hello", "hello!"));
}

#[test]
fn dot_and_quantifiers() {
    assert!(regex_match("h.llo", "hello"));
    assert!(regex_match("h.llo", "hallo"));
    assert!(regex_match("ab*c", "ac"));
    assert!(regex_match("ab*c", "abbc"));
    assert!(regex_match("ab+c", "abc"));
    assert!(!regex_match("ab+c", "ac"));
    assert!(regex_match("ab?c", "ac"));
    assert!(regex_match("ab?c", "abc"));
}

#[test]
fn alternation_and_grouping() {
    assert!(regex_match("cat|dog", "cat"));
    assert!(regex_match("cat|dog", "dog"));
    assert!(!regex_match("cat|dog", "cow"));
    assert!(regex_match("(ab)+", "abab"));
    assert!(!regex_match("(ab)+", ""));
}

#[test]
fn character_classes() {
    assert!(regex_match("[abc]", "a"));
    assert!(regex_match("[abc]", "c"));
    assert!(!regex_match("[abc]", "d"));
    assert!(regex_match("[a-z]+", "hello"));
    assert!(!regex_match("[a-z]+", "HELLO"));
}

#[test]
fn find_in_text() {
    let result = regex_find("world", "hello world!");
    assert_eq!(result, Some((6, 11)));

    let result2 = regex_find("[0-9]+", "abc 42 def");
    assert_eq!(result2, Some((4, 6)));

    let result3 = regex_find("xyz", "hello");
    assert_eq!(result3, None);
}

#[test]
fn negated_character_class() {
    assert!(regex_match("[^0-9]+", "hello"));
    assert!(!regex_match("[^0-9]+", "123"));
    assert!(regex_match("[^abc]", "d"));
    assert!(!regex_match("[^abc]", "a"));
}

#[test]
fn complex_patterns() {
    assert!(regex_match("a(b|c)*d", "ad"));
    assert!(regex_match("a(b|c)*d", "abcbd"));
    assert!(!regex_match("a(b|c)*d", "aed"));
}

#[test]
fn quantifier_on_group() {
    assert!(regex_match("(abc)+", "abcabc"));
    assert!(!regex_match("(abc)+", "abab"));
    assert!(regex_match("(a|b)+c", "aabbac"));
    assert!(!regex_match("(a|b)+c", "c"));
}

#[test]
fn empty_and_edge_cases() {
    assert!(regex_match("a*", ""));
    assert!(regex_match("a*", "aaa"));
    assert!(!regex_match("a+", ""));
    assert!(regex_match("(a*)(b*)", "aabb"));
}
