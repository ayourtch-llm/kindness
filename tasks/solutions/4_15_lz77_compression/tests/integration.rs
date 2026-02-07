use solution::*;

#[test]
fn test_empty() {
    let compressed = compress(b"", 256);
    assert!(compressed.is_empty());
    let decompressed = decompress(&compressed);
    assert!(decompressed.is_empty());
}

#[test]
fn test_no_repeats() {
    let input = b"abcdef";
    let compressed = compress(input, 256);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, input);
}

#[test]
fn test_all_same() {
    let input = b"aaaaaaaaaa"; // 10 a's
    let compressed = compress(input, 256);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, input);
    // Should have some Match tokens since there are repeats
    let match_count = compressed.iter().filter(|t| matches!(t, LzToken::Match { .. })).count();
    assert!(match_count > 0, "Expected at least one match token for repeated data");
}

#[test]
fn test_repeated_pattern() {
    let input = b"abcabcabcabc";
    let compressed = compress(input, 256);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, input);
}

#[test]
fn test_roundtrip_long() {
    let input: Vec<u8> = (0..200).map(|i| (i % 26 + 65) as u8).collect();
    let compressed = compress(&input, 128);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, input);
}

#[test]
fn test_small_window() {
    let input = b"abcdefabcdef";
    let compressed = compress(input, 6);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, input);
}

#[test]
fn test_decompress_manual() {
    // Manual tokens: "abc" as literals, then match back 3, length 3 => "abcabc"
    let tokens = vec![
        LzToken::Literal(b'a'),
        LzToken::Literal(b'b'),
        LzToken::Literal(b'c'),
        LzToken::Match { offset: 3, length: 3 },
    ];
    let result = decompress(&tokens);
    assert_eq!(result, b"abcabc");
}

#[test]
fn test_overlapping_match() {
    // Overlapping copy: "a" then match offset=1, length=5 => "aaaaaa"
    let tokens = vec![
        LzToken::Literal(b'a'),
        LzToken::Match { offset: 1, length: 5 },
    ];
    let result = decompress(&tokens);
    assert_eq!(result, b"aaaaaa");
}
