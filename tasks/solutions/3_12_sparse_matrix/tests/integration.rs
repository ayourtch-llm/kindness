use solution::*;

#[test]
fn test_new_and_get_zero() {
    let m = SparseMatrix::new(3, 3);
    assert!((m.get(0, 0) - 0.0).abs() < 1e-9);
    assert!((m.get(2, 2) - 0.0).abs() < 1e-9);
}

#[test]
fn test_set_and_get() {
    let mut m = SparseMatrix::new(3, 4);
    m.set(0, 1, 5.0);
    m.set(2, 3, -3.5);
    assert!((m.get(0, 1) - 5.0).abs() < 1e-9);
    assert!((m.get(2, 3) - (-3.5)).abs() < 1e-9);
    assert!((m.get(0, 0) - 0.0).abs() < 1e-9);
}

#[test]
fn test_set_zero_removes_entry() {
    let mut m = SparseMatrix::new(2, 2);
    m.set(0, 0, 42.0);
    assert!((m.get(0, 0) - 42.0).abs() < 1e-9);
    m.set(0, 0, 0.0);
    assert!((m.get(0, 0) - 0.0).abs() < 1e-9);
}

#[test]
fn test_transpose() {
    let mut m = SparseMatrix::new(2, 3);
    m.set(0, 1, 7.0);
    m.set(1, 2, 4.0);
    let t = m.transpose();
    assert!((t.get(1, 0) - 7.0).abs() < 1e-9);
    assert!((t.get(2, 1) - 4.0).abs() < 1e-9);
    assert!((t.get(0, 0) - 0.0).abs() < 1e-9);
}

#[test]
fn test_multiply_identity() {
    let mut a = SparseMatrix::new(2, 2);
    a.set(0, 0, 3.0);
    a.set(0, 1, 4.0);
    a.set(1, 0, 1.0);
    a.set(1, 1, 2.0);
    let mut identity = SparseMatrix::new(2, 2);
    identity.set(0, 0, 1.0);
    identity.set(1, 1, 1.0);
    let result = a.multiply(&identity);
    assert!((result.get(0, 0) - 3.0).abs() < 1e-9);
    assert!((result.get(0, 1) - 4.0).abs() < 1e-9);
    assert!((result.get(1, 0) - 1.0).abs() < 1e-9);
    assert!((result.get(1, 1) - 2.0).abs() < 1e-9);
}

#[test]
fn test_multiply_rectangular() {
    let mut a = SparseMatrix::new(2, 3);
    a.set(0, 0, 1.0);
    a.set(0, 2, 2.0);
    a.set(1, 1, 3.0);
    let mut b = SparseMatrix::new(3, 2);
    b.set(0, 0, 4.0);
    b.set(1, 1, 5.0);
    b.set(2, 0, 6.0);
    let c = a.multiply(&b);
    assert!((c.get(0, 0) - 16.0).abs() < 1e-9);
    assert!((c.get(0, 1) - 0.0).abs() < 1e-9);
    assert!((c.get(1, 0) - 0.0).abs() < 1e-9);
    assert!((c.get(1, 1) - 15.0).abs() < 1e-9);
}

#[test]
fn test_overwrite_value() {
    let mut m = SparseMatrix::new(2, 2);
    m.set(0, 0, 1.0);
    m.set(0, 0, 99.0);
    assert!((m.get(0, 0) - 99.0).abs() < 1e-9);
}
