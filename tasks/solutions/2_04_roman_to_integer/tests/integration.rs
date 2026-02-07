use solution::*;

#[test]
fn test_simple() {
    assert_eq!(roman_to_int("III"), 3);
}

#[test]
fn test_subtractive() {
    assert_eq!(roman_to_int("IV"), 4);
}

#[test]
fn test_complex() {
    assert_eq!(roman_to_int("MCMXCIV"), 1994);
}

#[test]
fn test_large() {
    assert_eq!(roman_to_int("MMMCMXCIX"), 3999);
}

#[test]
fn test_single_char() {
    assert_eq!(roman_to_int("D"), 500);
}

#[test]
fn test_lviii() {
    assert_eq!(roman_to_int("LVIII"), 58);
}
