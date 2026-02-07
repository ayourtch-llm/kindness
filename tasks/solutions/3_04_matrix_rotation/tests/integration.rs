use solution::*;

#[test]
fn test_3x3() {
    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    rotate(&mut matrix);
    assert_eq!(matrix, vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ]);
}

#[test]
fn test_4x4() {
    let mut matrix = vec![
        vec![1,  2,  3,  4],
        vec![5,  6,  7,  8],
        vec![9,  10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    rotate(&mut matrix);
    assert_eq!(matrix, vec![
        vec![13, 9,  5, 1],
        vec![14, 10, 6, 2],
        vec![15, 11, 7, 3],
        vec![16, 12, 8, 4],
    ]);
}

#[test]
fn test_1x1() {
    let mut matrix = vec![vec![42]];
    rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![42]]);
}

#[test]
fn test_2x2() {
    let mut matrix = vec![
        vec![1, 2],
        vec![3, 4],
    ];
    rotate(&mut matrix);
    assert_eq!(matrix, vec![
        vec![3, 1],
        vec![4, 2],
    ]);
}

#[test]
fn test_rotate_four_times_identity() {
    let original = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let mut matrix = original.clone();
    rotate(&mut matrix);
    rotate(&mut matrix);
    rotate(&mut matrix);
    rotate(&mut matrix);
    assert_eq!(matrix, original);
}
