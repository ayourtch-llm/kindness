use solution::*;

#[test]
fn test_basic() {
    assert_eq!(flatten(vec![vec![1, 2], vec![3, 4], vec![5]]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_empty_outer() {
    let empty: Vec<Vec<i32>> = vec![];
    assert_eq!(flatten(empty), vec![]);
}

#[test]
fn test_empty_inner_vecs() {
    assert_eq!(flatten(vec![vec![], vec![], vec![]]), vec![]);
}

#[test]
fn test_mixed_empty_and_nonempty() {
    assert_eq!(flatten(vec![vec![], vec![1], vec![], vec![2, 3]]), vec![1, 2, 3]);
}

#[test]
fn test_single_inner() {
    assert_eq!(flatten(vec![vec![10, 20, 30]]), vec![10, 20, 30]);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(flatten(vec![vec![-1, -2], vec![0], vec![1, 2]]), vec![-1, -2, 0, 1, 2]);
}
