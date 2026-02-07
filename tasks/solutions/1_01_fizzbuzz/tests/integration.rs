use solution::*;

#[test]
fn test_fizzbuzz_1() {
    assert_eq!(fizzbuzz(1), vec!["1"]);
}

#[test]
fn test_fizzbuzz_5() {
    assert_eq!(fizzbuzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
}

#[test]
fn test_fizzbuzz_15() {
    let result = fizzbuzz(15);
    assert_eq!(result.len(), 15);
    assert_eq!(result[2], "Fizz");
    assert_eq!(result[4], "Buzz");
    assert_eq!(result[14], "FizzBuzz");
}

#[test]
fn test_fizzbuzz_0() {
    assert_eq!(fizzbuzz(0), Vec::<String>::new());
}
