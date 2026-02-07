pub fn hamming_distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(ca, cb)| ca != cb).count()
}
