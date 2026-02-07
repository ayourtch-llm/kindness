use solution::*;

#[test]
fn test_suffix_array_simple() {
    let sa = build_suffix_array("banana");
    // Suffixes sorted: "a"(5), "ana"(3), "anana"(1), "banana"(0), "na"(4), "nana"(2)
    assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
}

#[test]
fn test_search_single_char() {
    let text = "abracadabra";
    let sa = build_suffix_array(text);
    let mut positions = search_pattern(text, &sa, "a");
    positions.sort();
    assert_eq!(positions, vec![0, 3, 5, 7, 10]);
}

#[test]
fn test_search_substring() {
    let text = "mississippi";
    let sa = build_suffix_array(text);
    let mut positions = search_pattern(text, &sa, "issi");
    positions.sort();
    assert_eq!(positions, vec![1, 4]);
}

#[test]
fn test_search_not_found() {
    let text = "hello";
    let sa = build_suffix_array(text);
    let positions = search_pattern(text, &sa, "xyz");
    assert!(positions.is_empty());
}

#[test]
fn test_search_full_string() {
    let text = "abcdef";
    let sa = build_suffix_array(text);
    let positions = search_pattern(text, &sa, "abcdef");
    assert_eq!(positions, vec![0]);
}

#[test]
fn test_single_char_text() {
    let sa = build_suffix_array("a");
    assert_eq!(sa, vec![0]);
    let positions = search_pattern("a", &sa, "a");
    assert_eq!(positions, vec![0]);
}

#[test]
fn test_repeated_pattern() {
    let text = "aaaaaa";
    let sa = build_suffix_array(text);
    let mut positions = search_pattern(text, &sa, "aa");
    positions.sort();
    assert_eq!(positions, vec![0, 1, 2, 3, 4]);
}
