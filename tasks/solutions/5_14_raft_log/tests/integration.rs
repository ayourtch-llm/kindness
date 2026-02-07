use solution::*;

#[test]
fn append_and_commit() {
    let mut node = RaftNode::new(1);
    node.set_term(1);
    let idx1 = node.append_entry(b"hello".to_vec());
    let idx2 = node.append_entry(b"world".to_vec());
    assert_eq!(idx1, 1);
    assert_eq!(idx2, 2);
    assert_eq!(node.last_log_index(), 2);
    assert_eq!(node.last_log_term(), 1);

    node.commit_up_to(2);
    let committed = node.get_committed();
    assert_eq!(committed.len(), 2);
    assert_eq!(committed[0].data, b"hello");
    assert_eq!(committed[1].data, b"world");
}

#[test]
fn receive_entries_success() {
    let mut node = RaftNode::new(1);
    node.set_term(1);
    node.append_entry(b"first".to_vec());

    let entries = vec![
        LogEntry { term: 1, index: 2, data: b"second".to_vec() },
        LogEntry { term: 1, index: 3, data: b"third".to_vec() },
    ];
    let ok = node.receive_append_entries(1, 1, 1, entries);
    assert!(ok);
    assert_eq!(node.last_log_index(), 3);
}

#[test]
fn reject_stale_term() {
    let mut node = RaftNode::new(1);
    node.set_term(5);
    let ok = node.receive_append_entries(3, 0, 0, vec![
        LogEntry { term: 3, index: 1, data: b"old".to_vec() },
    ]);
    assert!(!ok);
}

#[test]
fn reject_log_mismatch() {
    let mut node = RaftNode::new(1);
    node.set_term(1);
    node.append_entry(b"a".to_vec()); // index 1, term 1

    // Leader says prev_log_index=1, prev_log_term=2 — mismatch (node has term 1)
    let ok = node.receive_append_entries(2, 1, 2, vec![
        LogEntry { term: 2, index: 2, data: b"b".to_vec() },
    ]);
    assert!(!ok);
}

#[test]
fn overwrite_conflicting_entries() {
    let mut node = RaftNode::new(1);
    node.set_term(1);
    node.append_entry(b"a".to_vec()); // index 1, term 1
    node.append_entry(b"b".to_vec()); // index 2, term 1
    node.append_entry(b"c".to_vec()); // index 3, term 1

    // Leader at term 2 says: after index 1 (term 1), apply new entries
    let ok = node.receive_append_entries(2, 1, 1, vec![
        LogEntry { term: 2, index: 2, data: b"x".to_vec() },
        LogEntry { term: 2, index: 3, data: b"y".to_vec() },
    ]);
    assert!(ok);
    assert_eq!(node.last_log_index(), 3);

    node.commit_up_to(3);
    let committed = node.get_committed();
    assert_eq!(committed[1].data, b"x");
    assert_eq!(committed[1].term, 2);
    assert_eq!(committed[2].data, b"y");
}

#[test]
fn commit_capped_at_log_length() {
    let mut node = RaftNode::new(1);
    node.set_term(1);
    node.append_entry(b"only".to_vec());
    node.commit_up_to(100);
    let committed = node.get_committed();
    assert_eq!(committed.len(), 1);
}

#[test]
fn empty_node() {
    let node = RaftNode::new(1);
    assert_eq!(node.last_log_index(), 0);
    assert_eq!(node.last_log_term(), 0);
    assert_eq!(node.current_term(), 0);
    assert!(node.get_committed().is_empty());
}
