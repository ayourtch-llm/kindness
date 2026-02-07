pub fn digital_root(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    1 + (n - 1) % 9
}
