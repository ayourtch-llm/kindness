use solution::*;

#[test]
fn test_fib_0() {
    assert_eq!(fibonacci(0), 0);
}

#[test]
fn test_fib_1() {
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn test_fib_10() {
    assert_eq!(fibonacci(10), 55);
}

#[test]
fn test_fib_20() {
    assert_eq!(fibonacci(20), 6765);
}
