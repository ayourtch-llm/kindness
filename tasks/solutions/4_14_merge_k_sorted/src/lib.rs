use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn merge_k_sorted(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    for (i, list) in lists.iter().enumerate() {
        if !list.is_empty() {
            heap.push(Reverse((list[0], i, 0)));
        }
    }

    let mut result = Vec::new();
    while let Some(Reverse((val, list_idx, elem_idx))) = heap.pop() {
        result.push(val);
        let next_idx = elem_idx + 1;
        if next_idx < lists[list_idx].len() {
            heap.push(Reverse((lists[list_idx][next_idx], list_idx, next_idx)));
        }
    }

    result
}
