use std::collections::VecDeque;

pub fn topo_sort(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; num_nodes];
    let mut in_degree = vec![0usize; num_nodes];

    for &(u, v) in edges {
        adj[u].push(v);
        in_degree[v] += 1;
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 0..num_nodes {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut result = Vec::with_capacity(num_nodes);
    while let Some(node) = queue.pop_front() {
        result.push(node);
        for &neighbor in &adj[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if result.len() == num_nodes {
        Some(result)
    } else {
        None
    }
}
