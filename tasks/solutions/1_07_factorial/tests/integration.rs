use solution::*;

#[test]
fn test_zero() {
    assert_eq!(factorial(0), 1);
}

#[test]
fn test_one() {
    assert_eq!(factorial(1), 1);
}

#[test]
fn test_five() {
    assert_eq!(factorial(5), 120);
}

#[test]
fn test_ten() {
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn test_twenty() {
    assert_eq!(factorial(20), 2432902008176640000);
}
