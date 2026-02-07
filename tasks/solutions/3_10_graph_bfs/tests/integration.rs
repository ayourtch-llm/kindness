use solution::*;

#[test]
fn test_simple_graph() {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);
    let result = bfs(&graph, 1);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_single_node() {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(1, vec![]);
    let result = bfs(&graph, 1);
    assert_eq!(result, vec![1]);
}

#[test]
fn test_disconnected_start() {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(1, vec![2]);
    graph.insert(2, vec![]);
    let result = bfs(&graph, 99);
    assert_eq!(result, vec![]);
}

#[test]
fn test_cycle() {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![1]);
    let result = bfs(&graph, 1);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_diamond_graph() {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    let result = bfs(&graph, 0);
    assert_eq!(result, vec![0, 1, 2, 3]);
}
