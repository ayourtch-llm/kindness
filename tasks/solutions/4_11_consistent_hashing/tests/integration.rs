use solution::*;

#[test]
fn test_empty_ring() {
    let ring = HashRing::new(3);
    assert_eq!(ring.get_node("any_key"), None);
}

#[test]
fn test_single_node() {
    let mut ring = HashRing::new(3);
    ring.add_node("server1");
    assert_eq!(ring.get_node("key1"), Some("server1"));
    assert_eq!(ring.get_node("key2"), Some("server1"));
}

#[test]
fn test_multiple_nodes() {
    let mut ring = HashRing::new(100);
    ring.add_node("server1");
    ring.add_node("server2");
    ring.add_node("server3");
    // All keys should map to some node
    for i in 0..20 {
        let key = format!("key_{}", i);
        assert!(ring.get_node(&key).is_some());
    }
}

#[test]
fn test_remove_node() {
    let mut ring = HashRing::new(50);
    ring.add_node("A");
    ring.add_node("B");
    ring.add_node("C");
    ring.remove_node("B");
    // All keys should still resolve to A or C
    for i in 0..30 {
        let key = format!("test_{}", i);
        let node = ring.get_node(&key).unwrap();
        assert!(node == "A" || node == "C", "Unexpected node: {}", node);
    }
}

#[test]
fn test_consistency() {
    let mut ring = HashRing::new(50);
    ring.add_node("node1");
    ring.add_node("node2");
    // Same key always maps to same node
    let n1 = ring.get_node("stable_key").unwrap().to_string();
    let n2 = ring.get_node("stable_key").unwrap().to_string();
    assert_eq!(n1, n2);
}

#[test]
fn test_distribution() {
    let mut ring = HashRing::new(150);
    ring.add_node("A");
    ring.add_node("B");
    ring.add_node("C");
    let mut counts = std::collections::HashMap::new();
    for i in 0..3000 {
        let key = format!("key_{}", i);
        let node = ring.get_node(&key).unwrap();
        *counts.entry(node.to_string()).or_insert(0) += 1;
    }
    // Each node should get at least some keys (rough balance check)
    assert!(counts.get("A").unwrap_or(&0) > &100);
    assert!(counts.get("B").unwrap_or(&0) > &100);
    assert!(counts.get("C").unwrap_or(&0) > &100);
}

#[test]
fn test_minimal_disruption() {
    let mut ring = HashRing::new(150);
    ring.add_node("A");
    ring.add_node("B");

    let keys: Vec<String> = (0..200).map(|i| format!("k{}", i)).collect();
    let before: Vec<String> = keys.iter().map(|k| ring.get_node(k).unwrap().to_string()).collect();

    ring.add_node("C");
    let after: Vec<String> = keys.iter().map(|k| ring.get_node(k).unwrap().to_string()).collect();

    let unchanged = before.iter().zip(after.iter()).filter(|(a, b)| a == b).count();
    // Most keys should remain on the same node after adding one of three
    assert!(unchanged > 80, "Too many keys remapped: only {} unchanged out of 200", unchanged);
}
