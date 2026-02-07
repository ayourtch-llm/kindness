use solution::*;

#[test]
fn test_push_and_peek() {
    let mut heap = MinHeap::new();
    heap.push(5);
    heap.push(3);
    heap.push(8);
    assert_eq!(heap.peek(), Some(&3));
    assert_eq!(heap.len(), 3);
}

#[test]
fn test_pop_ordering() {
    let mut heap = MinHeap::new();
    heap.push(10);
    heap.push(4);
    heap.push(15);
    heap.push(1);
    heap.push(7);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), Some(10));
    assert_eq!(heap.pop(), Some(15));
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_empty_heap() {
    let mut heap: MinHeap<i32> = MinHeap::new();
    assert!(heap.is_empty());
    assert_eq!(heap.len(), 0);
    assert_eq!(heap.peek(), None);
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_single_element() {
    let mut heap = MinHeap::new();
    heap.push(42);
    assert_eq!(heap.peek(), Some(&42));
    assert_eq!(heap.len(), 1);
    assert!(!heap.is_empty());
    assert_eq!(heap.pop(), Some(42));
    assert!(heap.is_empty());
}

#[test]
fn test_duplicates() {
    let mut heap = MinHeap::new();
    heap.push(5);
    heap.push(5);
    heap.push(3);
    heap.push(3);
    heap.push(1);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(5));
}

#[test]
fn test_interleaved_push_pop() {
    let mut heap = MinHeap::new();
    heap.push(10);
    heap.push(20);
    assert_eq!(heap.pop(), Some(10));
    heap.push(5);
    assert_eq!(heap.peek(), Some(&5));
    heap.push(15);
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(15));
    assert_eq!(heap.pop(), Some(20));
    assert!(heap.is_empty());
}

#[test]
fn test_large_sequence() {
    let mut heap = MinHeap::new();
    for i in (0..100).rev() {
        heap.push(i);
    }
    assert_eq!(heap.len(), 100);
    for i in 0..100 {
        assert_eq!(heap.pop(), Some(i));
    }
    assert!(heap.is_empty());
}
