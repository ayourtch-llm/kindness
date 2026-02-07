use solution::*;

#[test]
fn test_empty() {
    assert_eq!(max_non_overlapping(&[]), 0);
}

#[test]
fn test_single() {
    assert_eq!(max_non_overlapping(&[(1, 5)]), 1);
}

#[test]
fn test_no_overlap() {
    assert_eq!(max_non_overlapping(&[(1, 2), (3, 4), (5, 6)]), 3);
}

#[test]
fn test_all_overlap() {
    assert_eq!(max_non_overlapping(&[(1, 10), (2, 9), (3, 8)]), 1);
}

#[test]
fn test_classic_greedy() {
    // (1,4) overlaps (2,5); pick (1,4) then (5,7) then (8,10)
    assert_eq!(max_non_overlapping(&[(1, 4), (2, 5), (5, 7), (6, 9), (8, 10)]), 3);
}

#[test]
fn test_touching_boundaries() {
    // Intervals that share an endpoint: (1,3) and (3,5) do NOT overlap
    assert_eq!(max_non_overlapping(&[(1, 3), (3, 5), (5, 7)]), 3);
}

#[test]
fn test_nested_intervals() {
    // (1,10) contains (2,3),(4,5),(6,7); skip outer, pick inner three
    assert_eq!(max_non_overlapping(&[(1, 10), (2, 3), (4, 5), (6, 7)]), 3);
}
