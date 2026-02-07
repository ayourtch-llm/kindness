pub fn zigzag_convert(s: &str, num_rows: usize) -> String {
    if num_rows <= 1 || num_rows >= s.len() {
        return s.to_string();
    }
    let mut rows = vec![String::new(); num_rows];
    let mut row: usize = 0;
    let mut going_down = false;
    for c in s.chars() {
        rows[row].push(c);
        if row == 0 || row == num_rows - 1 {
            going_down = !going_down;
        }
        if going_down {
            row += 1;
        } else {
            row -= 1;
        }
    }
    rows.concat()
}
