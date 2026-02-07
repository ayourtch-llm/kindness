pub fn find_median_sorted(nums1: &[i32], nums2: &[i32]) -> f64 {
    // Ensure binary search on the shorter array
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };
    let m = a.len();
    let n = b.len();
    let half = (m + n + 1) / 2;

    let mut lo: usize = 0;
    let mut hi: usize = m;

    while lo <= hi {
        let i = (lo + hi) / 2;
        let j = half - i;

        let a_left = if i == 0 { i32::MIN } else { a[i - 1] };
        let a_right = if i == m { i32::MAX } else { a[i] };
        let b_left = if j == 0 { i32::MIN } else { b[j - 1] };
        let b_right = if j == n { i32::MAX } else { b[j] };

        if a_left <= b_right && b_left <= a_right {
            if (m + n) % 2 == 1 {
                return a_left.max(b_left) as f64;
            } else {
                return (a_left.max(b_left) as f64 + a_right.min(b_right) as f64) / 2.0;
            }
        } else if a_left > b_right {
            hi = i - 1;
        } else {
            lo = i + 1;
        }
    }
    unreachable!()
}
