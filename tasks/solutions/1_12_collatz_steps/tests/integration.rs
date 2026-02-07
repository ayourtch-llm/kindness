use solution::*;

#[test]
fn test_one() {
    assert_eq!(collatz_steps(1), 0);
}

#[test]
fn test_two() {
    assert_eq!(collatz_steps(2), 1);
}

#[test]
fn test_six() {
    // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1  (8 steps)
    assert_eq!(collatz_steps(6), 8);
}

#[test]
fn test_classic_27() {
    // 27 is famous for taking 111 steps
    assert_eq!(collatz_steps(27), 111);
}

#[test]
fn test_power_of_two() {
    // 16 -> 8 -> 4 -> 2 -> 1  (4 steps)
    assert_eq!(collatz_steps(16), 4);
}
