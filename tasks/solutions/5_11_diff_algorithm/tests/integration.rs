use solution::*;

#[test]
fn identical_inputs() {
    let old = vec!["a", "b", "c"];
    let new = vec!["a", "b", "c"];
    let ops = diff(&old, &new);
    assert_eq!(ops, vec![
        DiffOp::Equal("a".into()),
        DiffOp::Equal("b".into()),
        DiffOp::Equal("c".into()),
    ]);
}

#[test]
fn simple_insertion() {
    let old = vec!["a", "c"];
    let new = vec!["a", "b", "c"];
    let ops = diff(&old, &new);
    assert_eq!(ops, vec![
        DiffOp::Equal("a".into()),
        DiffOp::Insert("b".into()),
        DiffOp::Equal("c".into()),
    ]);
}

#[test]
fn simple_deletion() {
    let old = vec!["a", "b", "c"];
    let new = vec!["a", "c"];
    let ops = diff(&old, &new);
    assert_eq!(ops, vec![
        DiffOp::Equal("a".into()),
        DiffOp::Delete("b".into()),
        DiffOp::Equal("c".into()),
    ]);
}

#[test]
fn replacement() {
    let old = vec!["a", "b", "c"];
    let new = vec!["a", "x", "c"];
    let ops = diff(&old, &new);
    // Minimal diff: delete b, insert x
    let has_delete_b = ops.contains(&DiffOp::Delete("b".into()));
    let has_insert_x = ops.contains(&DiffOp::Insert("x".into()));
    assert!(has_delete_b);
    assert!(has_insert_x);
    assert_eq!(ops.iter().filter(|o| matches!(o, DiffOp::Equal(_))).count(), 2);
}

#[test]
fn completely_different() {
    let old = vec!["a", "b"];
    let new = vec!["c", "d"];
    let ops = diff(&old, &new);
    let deletes: Vec<_> = ops.iter().filter(|o| matches!(o, DiffOp::Delete(_))).collect();
    let inserts: Vec<_> = ops.iter().filter(|o| matches!(o, DiffOp::Insert(_))).collect();
    assert_eq!(deletes.len(), 2);
    assert_eq!(inserts.len(), 2);
}

#[test]
fn format_unified_output() {
    let old = vec!["a", "b", "c", "d", "e"];
    let new = vec!["a", "b", "x", "d", "e"];
    let ops = diff(&old, &new);
    let unified = format_unified(&ops, 1);
    // Should contain context around the change
    assert!(unified.contains("+ x") || unified.contains("+x"));
    assert!(unified.contains("- c") || unified.contains("-c"));
    assert!(unified.contains("  b") || unified.contains(" b"));
}

#[test]
fn empty_inputs() {
    let old: Vec<&str> = vec![];
    let new: Vec<&str> = vec![];
    let ops = diff(&old, &new);
    assert!(ops.is_empty());

    let old2: Vec<&str> = vec![];
    let new2 = vec!["a"];
    let ops2 = diff(&old2, &new2);
    assert_eq!(ops2, vec![DiffOp::Insert("a".into())]);
}
