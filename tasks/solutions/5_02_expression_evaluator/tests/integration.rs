use solution::*;

#[test]
fn basic_arithmetic() {
    let eval = Evaluator::new();
    let r = eval.evaluate("2 + 3 * 4").unwrap();
    assert!((r - 14.0).abs() < 1e-9);
}

#[test]
fn operator_precedence_and_parens() {
    let eval = Evaluator::new();
    let r = eval.evaluate("(2 + 3) * 4").unwrap();
    assert!((r - 20.0).abs() < 1e-9);

    let r2 = eval.evaluate("2 ^ 3 ^ 2").unwrap();
    // right-associative: 2^(3^2) = 2^9 = 512
    assert!((r2 - 512.0).abs() < 1e-9);
}

#[test]
fn variables() {
    let mut eval = Evaluator::new();
    eval.set_variable("x", 10.0);
    eval.set_variable("y", 3.0);
    let r = eval.evaluate("x + y * 2").unwrap();
    assert!((r - 16.0).abs() < 1e-9);
}

#[test]
fn functions() {
    let mut eval = Evaluator::new();
    eval.register_function("double", Box::new(|args: &[f64]| args[0] * 2.0));
    eval.register_function("add", Box::new(|args: &[f64]| args[0] + args[1]));
    eval.set_variable("x", 5.0);
    let r = eval.evaluate("double(x) + add(1, 2)").unwrap();
    assert!((r - 13.0).abs() < 1e-9);
}

#[test]
fn unary_minus() {
    let eval = Evaluator::new();
    let r = eval.evaluate("-3 + 5").unwrap();
    assert!((r - 2.0).abs() < 1e-9);

    let r2 = eval.evaluate("-(2 + 3)").unwrap();
    assert!((r2 - (-5.0)).abs() < 1e-9);
}

#[test]
fn division_by_zero() {
    let eval = Evaluator::new();
    let r = eval.evaluate("1 / 0");
    assert!(r.is_err());
}

#[test]
fn unknown_variable_error() {
    let eval = Evaluator::new();
    let r = eval.evaluate("x + 1");
    assert!(r.is_err());
}

#[test]
fn nested_function_calls() {
    let mut eval = Evaluator::new();
    eval.register_function("double", Box::new(|args: &[f64]| args[0] * 2.0));
    eval.register_function("inc", Box::new(|args: &[f64]| args[0] + 1.0));
    let r = eval.evaluate("double(inc(3))").unwrap();
    assert!((r - 8.0).abs() < 1e-9);
}

#[test]
fn unknown_function_error() {
    let eval = Evaluator::new();
    let r = eval.evaluate("foo(1)");
    assert!(r.is_err());
}
