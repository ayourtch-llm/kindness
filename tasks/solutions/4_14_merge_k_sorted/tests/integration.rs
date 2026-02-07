use solution::*;

#[test]
fn test_empty_input() {
    let result = merge_k_sorted(vec![]);
    assert!(result.is_empty());
}

#[test]
fn test_single_list() {
    let result = merge_k_sorted(vec![vec![1, 3, 5, 7]]);
    assert_eq!(result, vec![1, 3, 5, 7]);
}

#[test]
fn test_two_lists() {
    let result = merge_k_sorted(vec![vec![1, 4, 7], vec![2, 5, 8]]);
    assert_eq!(result, vec![1, 2, 4, 5, 7, 8]);
}

#[test]
fn test_multiple_lists() {
    let result = merge_k_sorted(vec![
        vec![1, 10, 20],
        vec![2, 5, 50],
        vec![3, 15, 25],
        vec![4, 8, 30],
    ]);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 8, 10, 15, 20, 25, 30, 50]);
}

#[test]
fn test_with_empty_lists() {
    let result = merge_k_sorted(vec![vec![], vec![1, 2], vec![], vec![3, 4], vec![]]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_all_empty_lists() {
    let result = merge_k_sorted(vec![vec![], vec![], vec![]]);
    assert!(result.is_empty());
}

#[test]
fn test_duplicates_across_lists() {
    let result = merge_k_sorted(vec![vec![1, 3, 5], vec![1, 3, 5], vec![2, 4, 6]]);
    assert_eq!(result, vec![1, 1, 2, 3, 3, 4, 5, 5, 6]);
}

#[test]
fn test_negative_numbers() {
    let result = merge_k_sorted(vec![vec![-10, -5, 0], vec![-8, -3, 2], vec![-7, 1, 5]]);
    assert_eq!(result, vec![-10, -8, -7, -5, -3, 0, 1, 2, 5]);
}
