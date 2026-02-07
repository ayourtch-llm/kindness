pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}
