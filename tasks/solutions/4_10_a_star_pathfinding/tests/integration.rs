use solution::*;

#[test]
fn test_trivial() {
    let grid = vec![vec![true]];
    let path = astar(&grid, (0, 0), (0, 0)).unwrap();
    assert_eq!(path, vec![(0, 0)]);
}

#[test]
fn test_straight_line() {
    let grid = vec![vec![true; 5]];
    let path = astar(&grid, (0, 0), (0, 4)).unwrap();
    assert_eq!(path.len(), 5);
    assert_eq!(path[0], (0, 0));
    assert_eq!(path[4], (0, 4));
}

#[test]
fn test_around_wall() {
    let grid = vec![
        vec![true, true, true],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let path = astar(&grid, (0, 0), (2, 0)).unwrap();
    // Must go right, down, down, left, left
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(2, 0)));
    assert_eq!(path.len(), 7); // shortest path length
}

#[test]
fn test_no_path() {
    let grid = vec![
        vec![true, false, true],
        vec![true, false, true],
        vec![true, false, true],
    ];
    let result = astar(&grid, (0, 0), (0, 2));
    assert!(result.is_none());
}

#[test]
fn test_start_is_wall() {
    let grid = vec![
        vec![false, true],
        vec![true, true],
    ];
    let result = astar(&grid, (0, 0), (1, 1));
    assert!(result.is_none());
}

#[test]
fn test_larger_grid() {
    let grid = vec![
        vec![true,  true,  true,  true, true],
        vec![true,  false, false, false, true],
        vec![true,  true,  true,  false, true],
        vec![false, false, true,  false, true],
        vec![true,  true,  true,  true,  true],
    ];
    let path = astar(&grid, (0, 0), (4, 4)).unwrap();
    assert_eq!(path.first(), Some(&(0, 0)));
    assert_eq!(path.last(), Some(&(4, 4)));
    // Verify path is connected: each step differs by exactly 1 in one coordinate
    for w in path.windows(2) {
        let dr = (w[0].0 as i32 - w[1].0 as i32).abs();
        let dc = (w[0].1 as i32 - w[1].1 as i32).abs();
        assert_eq!(dr + dc, 1);
    }
}
