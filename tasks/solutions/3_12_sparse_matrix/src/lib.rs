pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    entries: Vec<(usize, usize, f64)>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        SparseMatrix {
            rows,
            cols,
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        // Remove existing entry at (row, col)
        self.entries.retain(|&(r, c, _)| !(r == row && c == col));
        if val.abs() > 1e-9 {
            self.entries.push((row, col, val));
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        for &(r, c, v) in &self.entries {
            if r == row && c == col {
                return v;
            }
        }
        0.0
    }

    pub fn transpose(&self) -> SparseMatrix {
        let mut result = SparseMatrix::new(self.cols, self.rows);
        for &(r, c, v) in &self.entries {
            result.entries.push((c, r, v));
        }
        result
    }

    pub fn multiply(&self, other: &SparseMatrix) -> SparseMatrix {
        assert_eq!(self.cols, other.rows, "Incompatible dimensions for multiplication");
        let mut result = SparseMatrix::new(self.rows, other.cols);
        for &(r1, c1, v1) in &self.entries {
            for &(r2, c2, v2) in &other.entries {
                if c1 == r2 {
                    let current = result.get(r1, c2);
                    let new_val = current + v1 * v2;
                    // Update directly in entries
                    result.entries.retain(|&(r, c, _)| !(r == r1 && c == c2));
                    if new_val.abs() > 1e-9 {
                        result.entries.push((r1, c2, new_val));
                    }
                }
            }
        }
        result
    }
}
