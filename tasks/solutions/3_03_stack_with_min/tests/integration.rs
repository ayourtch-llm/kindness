use solution::*;

#[test]
fn test_basic_operations() {
    let mut stack = MinStack::new();
    stack.push(3);
    stack.push(5);
    assert_eq!(stack.min(), Some(3));
    stack.push(2);
    assert_eq!(stack.min(), Some(2));
}

#[test]
fn test_pop_updates_min() {
    let mut stack = MinStack::new();
    stack.push(2);
    stack.push(1);
    stack.push(3);
    assert_eq!(stack.min(), Some(1));
    stack.pop();
    assert_eq!(stack.min(), Some(1));
    stack.pop();
    assert_eq!(stack.min(), Some(2));
}

#[test]
fn test_empty_stack() {
    let mut stack = MinStack::new();
    assert_eq!(stack.min(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_duplicate_minimums() {
    let mut stack = MinStack::new();
    stack.push(1);
    stack.push(1);
    stack.push(1);
    assert_eq!(stack.min(), Some(1));
    stack.pop();
    assert_eq!(stack.min(), Some(1));
    stack.pop();
    assert_eq!(stack.min(), Some(1));
    stack.pop();
    assert_eq!(stack.min(), None);
}

#[test]
fn test_decreasing_then_increasing() {
    let mut stack = MinStack::new();
    stack.push(5);
    stack.push(3);
    stack.push(1);
    assert_eq!(stack.min(), Some(1));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.min(), Some(3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.min(), Some(5));
}
