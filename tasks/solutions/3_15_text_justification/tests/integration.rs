use solution::*;

#[test]
fn test_basic_justification() {
    let words = vec!["This", "is", "an", "example", "of", "text", "justification."];
    let result = justify(&words, 16);
    assert_eq!(result, vec![
        "This    is    an",
        "example  of text",
        "justification.  ",
    ]);
}

#[test]
fn test_single_word_per_line() {
    let words = vec!["hello", "world"];
    let result = justify(&words, 5);
    assert_eq!(result, vec![
        "hello",
        "world",
    ]);
}

#[test]
fn test_last_line_left_justified() {
    let words = vec!["What", "must", "be", "acknowledgment", "shall", "be"];
    let result = justify(&words, 16);
    assert_eq!(result, vec![
        "What   must   be",
        "acknowledgment  ",
        "shall be        ",
    ]);
}

#[test]
fn test_long_sentence() {
    let words = vec![
        "Science", "is", "what", "we", "understand", "well", "enough",
        "to", "explain", "to", "a", "computer.", "Art", "is", "everything",
        "else", "we", "do",
    ];
    let result = justify(&words, 20);
    assert_eq!(result, vec![
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  ",
    ]);
}

#[test]
fn test_single_word_input() {
    let words = vec!["alone"];
    let result = justify(&words, 10);
    assert_eq!(result, vec!["alone     "]);
}

#[test]
fn test_exact_fit_no_extra_spaces() {
    let words = vec!["a", "b", "c"];
    let result = justify(&words, 5);
    assert_eq!(result, vec!["a b c"]);
}

#[test]
fn test_uneven_space_distribution() {
    let words = vec!["aa", "bb", "cc", "dd"];
    let result = justify(&words, 8);
    assert_eq!(result, vec![
        "aa bb cc",
        "dd      ",
    ]);
}
