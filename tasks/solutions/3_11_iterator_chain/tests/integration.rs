use solution::*;

#[test]
fn test_basic_map_filter() {
    let data = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = map_filter(
        data.into_iter(),
        |x| x * 10,
        |x| *x > 20,
    ).collect();
    assert_eq!(result, vec![30, 40, 50]);
}

#[test]
fn test_empty_iterator() {
    let data: Vec<i32> = vec![];
    let result: Vec<i32> = map_filter(
        data.into_iter(),
        |x| x + 1,
        |x| *x > 0,
    ).collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_all_filtered_out() {
    let data = vec![1, 2, 3];
    let result: Vec<i32> = map_filter(
        data.into_iter(),
        |x| x * 2,
        |x| *x > 100,
    ).collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_none_filtered_out() {
    let data = vec![10, 20, 30];
    let result: Vec<i32> = map_filter(
        data.into_iter(),
        |x| x + 1,
        |_x| true,
    ).collect();
    assert_eq!(result, vec![11, 21, 31]);
}

#[test]
fn test_type_conversion() {
    let data = vec![1, 2, 3, 4, 5];
    let result: Vec<String> = map_filter(
        data.into_iter(),
        |x| format!("item_{}", x),
        |s| s.ends_with('3') || s.ends_with('5'),
    ).collect();
    assert_eq!(result, vec!["item_3".to_string(), "item_5".to_string()]);
}

#[test]
fn test_laziness_with_side_effects() {
    use std::cell::Cell;
    let count = Cell::new(0u32);
    let data = vec![1, 2, 3, 4, 5];
    let mut iter = map_filter(
        data.into_iter(),
        |x| { count.set(count.get() + 1); x * 2 },
        |x| *x > 4,
    );
    assert_eq!(count.get(), 0);
    assert_eq!(iter.next(), Some(6));
    assert_eq!(count.get(), 3);
}

#[test]
fn test_chained_with_std_adapters() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result: Vec<i32> = map_filter(
        data.into_iter(),
        |x| x * x,
        |x| *x % 2 == 0,
    ).take(2).collect();
    assert_eq!(result, vec![4, 16]);
}
