pub fn max_non_overlapping(intervals: &[(i32, i32)]) -> usize {
    if intervals.is_empty() {
        return 0;
    }
    let mut sorted: Vec<(i32, i32)> = intervals.to_vec();
    sorted.sort_by_key(|&(_, end)| end);
    let mut count = 1;
    let mut last_end = sorted[0].1;
    for &(start, end) in &sorted[1..] {
        if start >= last_end {
            count += 1;
            last_end = end;
        }
    }
    count
}
