use solution::*;

#[test]
fn test_basic_3_rows() {
    assert_eq!(zigzag_convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
}

#[test]
fn test_basic_4_rows() {
    assert_eq!(zigzag_convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
}

#[test]
fn test_single_row() {
    assert_eq!(zigzag_convert("ABCDEF", 1), "ABCDEF");
}

#[test]
fn test_rows_exceed_length() {
    assert_eq!(zigzag_convert("AB", 5), "AB");
}

#[test]
fn test_two_rows() {
    assert_eq!(zigzag_convert("ABCDEF", 2), "ACEBDF");
}

#[test]
fn test_single_char() {
    assert_eq!(zigzag_convert("A", 1), "A");
}

#[test]
fn test_empty_string() {
    assert_eq!(zigzag_convert("", 3), "");
}
