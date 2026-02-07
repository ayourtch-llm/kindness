use solution::*;

#[test]
fn test_simple_odd() {
    let result = find_median_sorted(&[1, 3], &[2]);
    assert!((result - 2.0).abs() < 1e-9);
}

#[test]
fn test_simple_even() {
    let result = find_median_sorted(&[1, 2], &[3, 4]);
    assert!((result - 2.5).abs() < 1e-9);
}

#[test]
fn test_one_empty() {
    let result = find_median_sorted(&[], &[1, 2, 3, 4, 5]);
    assert!((result - 3.0).abs() < 1e-9);
}

#[test]
fn test_single_elements() {
    let result = find_median_sorted(&[1], &[2]);
    assert!((result - 1.5).abs() < 1e-9);
}

#[test]
fn test_non_overlapping() {
    let result = find_median_sorted(&[1, 2, 3], &[10, 20, 30]);
    assert!((result - 6.5).abs() < 1e-9);
}

#[test]
fn test_identical() {
    let result = find_median_sorted(&[5, 5, 5], &[5, 5, 5]);
    assert!((result - 5.0).abs() < 1e-9);
}

#[test]
fn test_large_disparity() {
    let result = find_median_sorted(&[1], &[2, 3, 4, 5, 6, 7, 8]);
    assert!((result - 4.5).abs() < 1e-9);
}
