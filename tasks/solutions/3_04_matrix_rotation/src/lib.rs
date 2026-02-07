pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    // Transpose
    for i in 0..n {
        for j in (i + 1)..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
    // Reverse each row
    for row in matrix.iter_mut() {
        row.reverse();
    }
}
