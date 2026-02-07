#[derive(Debug, Clone, PartialEq)]
pub enum DiffOp {
    Equal(String),
    Insert(String),
    Delete(String),
}

/// Myers diff algorithm: compute the shortest edit script between two sequences.
pub fn diff(old: &[&str], new: &[&str]) -> Vec<DiffOp> {
    let n = old.len();
    let m = new.len();

    if n == 0 && m == 0 {
        return vec![];
    }

    let max = n + m;
    // v[k + offset] stores the furthest reaching x for diagonal k
    let offset = max as isize;
    let vsize = 2 * max + 1;

    let mut v = vec![0isize; vsize];
    // Store the history of v arrays for backtracking
    let mut trace: Vec<Vec<isize>> = Vec::new();

    let mut found = false;
    for d in 0..=(max as isize) {
        trace.push(v.clone());
        let mut new_v = v.clone();
        for k in (-d..=d).step_by(2) {
            let idx = (k + offset) as usize;
            let mut x = if k == -d || (k != d && v[(k - 1 + offset) as usize] < v[(k + 1 + offset) as usize]) {
                v[(k + 1 + offset) as usize]
            } else {
                v[(k - 1 + offset) as usize] + 1
            };
            let mut y = x - k;

            while x < n as isize && y < m as isize && old[x as usize] == new[y as usize] {
                x += 1;
                y += 1;
            }

            new_v[idx] = x;

            if x >= n as isize && y >= m as isize {
                // Replace last trace entry with the final state
                let last = trace.len() - 1;
                trace[last] = new_v.clone();
                found = true;
                break;
            }
        }
        v = new_v;
        if found {
            break;
        }
    }

    // Backtrack to find the actual edit path
    let mut x = n as isize;
    let mut y = m as isize;
    let mut ops: Vec<DiffOp> = Vec::new();

    for d in (0..trace.len()).rev() {
        let v_d = &trace[d];
        let k = x - y;
        let d = d as isize;

        let prev_k = if k == -d || (k != d && v_d[((k - 1) + offset) as usize] < v_d[((k + 1) + offset) as usize]) {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v_d[(prev_k + offset) as usize];
        let prev_y = prev_x - prev_k;

        // Diagonal moves (equals)
        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;
            ops.push(DiffOp::Equal(old[x as usize].to_string()));
        }

        if d > 0 {
            if x == prev_x {
                // Insert
                y -= 1;
                ops.push(DiffOp::Insert(new[y as usize].to_string()));
            } else {
                // Delete
                x -= 1;
                ops.push(DiffOp::Delete(old[x as usize].to_string()));
            }
        }
    }

    ops.reverse();
    ops
}

/// Format diff operations as a unified diff string.
pub fn format_unified(ops: &[DiffOp], context: usize) -> String {
    if ops.is_empty() {
        return String::new();
    }

    // Find change indices
    let change_indices: Vec<usize> = ops
        .iter()
        .enumerate()
        .filter(|(_, op)| !matches!(op, DiffOp::Equal(_)))
        .map(|(i, _)| i)
        .collect();

    if change_indices.is_empty() {
        return String::new();
    }

    // Group changes into hunks based on context overlap
    let mut hunks: Vec<(usize, usize)> = Vec::new(); // (start, end) indices into ops
    let mut hunk_start = change_indices[0].saturating_sub(context);
    let mut hunk_end = (change_indices[0] + context + 1).min(ops.len());

    for &ci in &change_indices[1..] {
        let cs = ci.saturating_sub(context);
        let ce = (ci + context + 1).min(ops.len());
        if cs <= hunk_end {
            hunk_end = ce;
        } else {
            hunks.push((hunk_start, hunk_end));
            hunk_start = cs;
            hunk_end = ce;
        }
    }
    hunks.push((hunk_start, hunk_end));

    let mut result = String::new();
    for (i, (start, end)) in hunks.iter().enumerate() {
        if i > 0 {
            result.push_str("---\n");
        }
        for op in &ops[*start..*end] {
            match op {
                DiffOp::Equal(s) => {
                    result.push_str("  ");
                    result.push_str(s);
                    result.push('\n');
                }
                DiffOp::Insert(s) => {
                    result.push_str("+ ");
                    result.push_str(s);
                    result.push('\n');
                }
                DiffOp::Delete(s) => {
                    result.push_str("- ");
                    result.push_str(s);
                    result.push('\n');
                }
            }
        }
    }

    result
}
