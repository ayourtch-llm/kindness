pub fn build_suffix_array(text: &str) -> Vec<usize> {
    let n = text.len();
    if n == 0 {
        return vec![];
    }
    let bytes = text.as_bytes();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| bytes[a..].cmp(&bytes[b..]));
    sa
}

pub fn search_pattern(text: &str, sa: &[usize], pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || sa.is_empty() {
        return vec![];
    }
    let text_bytes = text.as_bytes();
    let pat_bytes = pattern.as_bytes();
    let n = sa.len();
    let m = pat_bytes.len();

    // Find lower bound
    let lo = {
        let mut lo = 0usize;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let suffix = &text_bytes[sa[mid]..];
            let cmp_len = m.min(suffix.len());
            if suffix[..cmp_len] < *pat_bytes {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    };

    // Find upper bound
    let hi = {
        let mut lo_b = lo;
        let mut hi_b = n;
        while lo_b < hi_b {
            let mid = lo_b + (hi_b - lo_b) / 2;
            let suffix = &text_bytes[sa[mid]..];
            let cmp_len = m.min(suffix.len());
            if suffix[..cmp_len] <= *pat_bytes {
                lo_b = mid + 1;
            } else {
                hi_b = mid;
            }
        }
        lo_b
    };

    let mut result: Vec<usize> = sa[lo..hi].to_vec();
    result.sort();
    result
}
