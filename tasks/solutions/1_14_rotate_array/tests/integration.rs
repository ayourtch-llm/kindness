use solution::*;

#[test]
fn test_basic_rotate() {
    let mut v = vec![1, 2, 3, 4, 5];
    rotate_left(&mut v, 2);
    assert_eq!(v, vec![3, 4, 5, 1, 2]);
}

#[test]
fn test_rotate_by_zero() {
    let mut v = vec![1, 2, 3];
    rotate_left(&mut v, 0);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn test_rotate_by_len() {
    let mut v = vec![1, 2, 3, 4];
    rotate_left(&mut v, 4);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_rotate_k_greater_than_len() {
    let mut v = vec![10, 20, 30];
    rotate_left(&mut v, 7); // 7 % 3 == 1
    assert_eq!(v, vec![20, 30, 10]);
}

#[test]
fn test_empty_slice() {
    let mut v: Vec<i32> = vec![];
    rotate_left(&mut v, 3);
    assert_eq!(v, vec![]);
}

#[test]
fn test_single_element() {
    let mut v = vec![42];
    rotate_left(&mut v, 5);
    assert_eq!(v, vec![42]);
}
