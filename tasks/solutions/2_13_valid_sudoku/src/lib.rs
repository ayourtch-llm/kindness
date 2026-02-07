pub fn is_valid_sudoku(board: &[[char; 9]; 9]) -> bool {
    use std::collections::HashSet;
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];
    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }
            let box_idx = (i / 3) * 3 + j / 3;
            if !rows[i].insert(c) || !cols[j].insert(c) || !boxes[box_idx].insert(c) {
                return false;
            }
        }
    }
    true
}
