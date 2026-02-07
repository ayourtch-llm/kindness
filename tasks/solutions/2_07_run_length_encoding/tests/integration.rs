use solution::*;

#[test]
fn test_basic_encoding() {
    assert_eq!(rle_encode("aaabbc"), "a3b2c");
}

#[test]
fn test_no_repeats() {
    assert_eq!(rle_encode("abcd"), "abcd");
}

#[test]
fn test_all_same() {
    assert_eq!(rle_encode("aaaa"), "a4");
}

#[test]
fn test_empty_string() {
    assert_eq!(rle_encode(""), "");
}

#[test]
fn test_single_char() {
    assert_eq!(rle_encode("z"), "z");
}

#[test]
fn test_mixed_runs() {
    assert_eq!(rle_encode("wwwwaaadexxxxxxywww"), "w4a3dex6yw3");
}
