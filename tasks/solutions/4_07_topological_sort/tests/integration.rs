use solution::*;

#[test]
fn test_empty_graph() {
    let result = topo_sort(0, &[]);
    assert_eq!(result, Some(vec![]));
}

#[test]
fn test_single_node() {
    let result = topo_sort(1, &[]);
    assert_eq!(result, Some(vec![0]));
}

#[test]
fn test_linear_chain() {
    let result = topo_sort(4, &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(result, Some(vec![0, 1, 2, 3]));
}

#[test]
fn test_cycle_detection() {
    let result = topo_sort(3, &[(0, 1), (1, 2), (2, 0)]);
    assert_eq!(result, None);
}

#[test]
fn test_diamond() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3
    let result = topo_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]).unwrap();
    assert_eq!(result[0], 0);
    assert_eq!(result[3], 3);
    // 1 and 2 can be in either order
    assert!(result.contains(&1));
    assert!(result.contains(&2));
}

#[test]
fn test_disconnected() {
    // Two separate components: 0->1 and 2->3
    let result = topo_sort(4, &[(0, 1), (2, 3)]).unwrap();
    assert_eq!(result.len(), 4);
    // 0 before 1, 2 before 3
    let pos = |x: usize| result.iter().position(|&n| n == x).unwrap();
    assert!(pos(0) < pos(1));
    assert!(pos(2) < pos(3));
}

#[test]
fn test_self_loop() {
    let result = topo_sort(2, &[(0, 0)]);
    assert_eq!(result, None);
}
