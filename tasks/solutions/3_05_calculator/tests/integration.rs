use solution::*;

#[test]
fn test_simple_addition() {
    let result = calculate("2 + 3");
    assert!((result - 5.0).abs() < 1e-9);
}

#[test]
fn test_operator_precedence() {
    let result = calculate("2 + 3 * 4");
    assert!((result - 14.0).abs() < 1e-9);
}

#[test]
fn test_parentheses() {
    let result = calculate("(2 + 3) * 4");
    assert!((result - 20.0).abs() < 1e-9);
}

#[test]
fn test_nested_parentheses() {
    let result = calculate("((2 + 3) * (4 - 1)) / 5");
    assert!((result - 3.0).abs() < 1e-9);
}

#[test]
fn test_division() {
    let result = calculate("10 / 4");
    assert!((result - 2.5).abs() < 1e-9);
}

#[test]
fn test_complex_expression() {
    let result = calculate("3 + 4 * 2 / (1 - 5) * 2 + 3");
    assert!((result - 2.0).abs() < 1e-9);
}

#[test]
fn test_unary_minus() {
    let result = calculate("-3 + 5");
    assert!((result - 2.0).abs() < 1e-9);
    let result2 = calculate("-(2 + 3)");
    assert!((result2 - (-5.0)).abs() < 1e-9);
    let result3 = calculate("2 * -3");
    assert!((result3 - (-6.0)).abs() < 1e-9);
}

#[test]
fn test_subtraction_chain() {
    let result = calculate("10 - 3 - 2");
    assert!((result - 5.0).abs() < 1e-9);
}
