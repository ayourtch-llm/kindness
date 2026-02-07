use solution::*;

#[test]
fn test_empty() {
    let sl = SkipList::new();
    assert_eq!(sl.len(), 0);
    assert!(!sl.contains(1));
}

#[test]
fn test_insert_and_contains() {
    let mut sl = SkipList::new();
    assert!(sl.insert(5));
    assert!(sl.insert(3));
    assert!(sl.insert(7));
    assert!(sl.contains(5));
    assert!(sl.contains(3));
    assert!(sl.contains(7));
    assert!(!sl.contains(4));
    assert_eq!(sl.len(), 3);
}

#[test]
fn test_no_duplicates() {
    let mut sl = SkipList::new();
    assert!(sl.insert(10));
    assert!(!sl.insert(10)); // duplicate
    assert_eq!(sl.len(), 1);
}

#[test]
fn test_remove() {
    let mut sl = SkipList::new();
    sl.insert(1);
    sl.insert(2);
    sl.insert(3);
    assert!(sl.remove(2));
    assert!(!sl.contains(2));
    assert_eq!(sl.len(), 2);
    assert!(!sl.remove(2)); // already removed
}

#[test]
fn test_many_elements() {
    let mut sl = SkipList::new();
    for i in 0..100 {
        assert!(sl.insert(i));
    }
    assert_eq!(sl.len(), 100);
    for i in 0..100 {
        assert!(sl.contains(i));
    }
    for i in 0..50 {
        assert!(sl.remove(i));
    }
    assert_eq!(sl.len(), 50);
    for i in 0..50 {
        assert!(!sl.contains(i));
    }
    for i in 50..100 {
        assert!(sl.contains(i));
    }
}

#[test]
fn test_negative_values() {
    let mut sl = SkipList::new();
    sl.insert(-5);
    sl.insert(0);
    sl.insert(5);
    assert!(sl.contains(-5));
    assert!(sl.contains(0));
    assert!(sl.contains(5));
    assert_eq!(sl.len(), 3);
}
