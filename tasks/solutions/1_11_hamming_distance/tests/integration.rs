use solution::*;

#[test]
fn test_identical() {
    assert_eq!(hamming_distance("karolin", "karolin"), 0);
}

#[test]
fn test_one_diff() {
    assert_eq!(hamming_distance("abc", "abc"), 0);
    assert_eq!(hamming_distance("abc", "axc"), 1);
}

#[test]
fn test_all_differ() {
    assert_eq!(hamming_distance("abc", "xyz"), 3);
}

#[test]
fn test_classic_example() {
    assert_eq!(hamming_distance("karolin", "kathrin"), 3);
}

#[test]
fn test_empty_strings() {
    assert_eq!(hamming_distance("", ""), 0);
}

#[test]
fn test_binary_strings() {
    assert_eq!(hamming_distance("10101", "01010"), 5);
    assert_eq!(hamming_distance("11011", "11010"), 1);
}
