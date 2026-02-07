use solution::*;

#[test]
fn test_push_front_and_to_vec() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_push_back_and_to_vec() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_pop_front() {
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_len() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    list.push_back(1);
    assert_eq!(list.len(), 1);
    list.push_front(2);
    assert_eq!(list.len(), 2);
    list.pop_front();
    assert_eq!(list.len(), 1);
}

#[test]
fn test_mixed_operations() {
    let mut list = LinkedList::new();
    list.push_back(2);
    list.push_front(1);
    list.push_back(3);
    list.push_front(0);
    assert_eq!(list.to_vec(), vec![0, 1, 2, 3]);
    assert_eq!(list.len(), 4);
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}
