use solution::*;

#[test]
fn test_3x3() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    assert_eq!(spiral_order(&matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
}

#[test]
fn test_3x4() {
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    assert_eq!(spiral_order(&matrix), vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
}

#[test]
fn test_single_row() {
    let matrix = vec![vec![1, 2, 3, 4]];
    assert_eq!(spiral_order(&matrix), vec![1, 2, 3, 4]);
}

#[test]
fn test_single_column() {
    let matrix = vec![vec![1], vec![2], vec![3]];
    assert_eq!(spiral_order(&matrix), vec![1, 2, 3]);
}

#[test]
fn test_empty() {
    let matrix: Vec<Vec<i32>> = vec![];
    assert_eq!(spiral_order(&matrix), vec![]);
}

#[test]
fn test_single_element() {
    let matrix = vec![vec![42]];
    assert_eq!(spiral_order(&matrix), vec![42]);
}

#[test]
fn test_2x2() {
    let matrix = vec![
        vec![1, 2],
        vec![3, 4],
    ];
    assert_eq!(spiral_order(&matrix), vec![1, 2, 4, 3]);
}
