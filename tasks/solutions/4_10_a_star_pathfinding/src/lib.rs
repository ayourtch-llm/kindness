use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct State {
    f: usize,
    g: usize,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f).then_with(|| other.g.cmp(&self.g))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).unsigned_abs() + (a.1 as isize - b.1 as isize).unsigned_abs())
}

pub fn astar(
    grid: &[Vec<bool>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    if !grid[start.0][start.1] || !grid[goal.0][goal.1] {
        return None;
    }
    if start == goal {
        return Some(vec![start]);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut open = BinaryHeap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    g_score.insert(start, 0);
    open.push(State {
        f: heuristic(start, goal),
        g: 0,
        pos: start,
    });

    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(State { g, pos, .. }) = open.pop() {
        if pos == goal {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some(path);
        }

        if g > *g_score.get(&pos).unwrap_or(&usize::MAX) {
            continue;
        }

        for &(dr, dc) in &dirs {
            let nr = pos.0 as isize + dr;
            let nc = pos.1 as isize + dc;
            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                continue;
            }
            let next = (nr as usize, nc as usize);
            if !grid[next.0][next.1] {
                continue;
            }
            let new_g = g + 1;
            if new_g < *g_score.get(&next).unwrap_or(&usize::MAX) {
                g_score.insert(next, new_g);
                came_from.insert(next, pos);
                open.push(State {
                    f: new_g + heuristic(next, goal),
                    g: new_g,
                    pos: next,
                });
            }
        }
    }

    None
}
