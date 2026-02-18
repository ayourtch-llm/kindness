use solution::*;

#[test]
fn literal_types() {
    use std::collections::HashMap;

    let env = HashMap::new();
    assert_eq!(type_check(&Expr::Lit(42), &env).unwrap(), Type::Int);
    assert_eq!(type_check(&Expr::BoolLit(true), &env).unwrap(), Type::Bool);
}

#[test]
fn add_and_eq() {
    use std::collections::HashMap;

    let env = HashMap::new();
    let add_expr = Expr::Add(
        Box::new(Expr::Lit(1)),
        Box::new(Expr::Lit(2)),
    );
    assert_eq!(type_check(&add_expr, &env).unwrap(), Type::Int);

    let eq_expr = Expr::Eq(
        Box::new(Expr::Lit(1)),
        Box::new(Expr::Lit(2)),
    );
    assert_eq!(type_check(&eq_expr, &env).unwrap(), Type::Bool);

    // Type error: adding int and bool
    let bad_add = Expr::Add(
        Box::new(Expr::Lit(1)),
        Box::new(Expr::BoolLit(true)),
    );
    assert!(type_check(&bad_add, &env).is_err());
}

#[test]
fn if_expression() {
    use std::collections::HashMap;

    let env = HashMap::new();
    let if_expr = Expr::If(
        Box::new(Expr::BoolLit(true)),
        Box::new(Expr::Lit(1)),
        Box::new(Expr::Lit(2)),
    );
    assert_eq!(type_check(&if_expr, &env).unwrap(), Type::Int);

    // Error: condition is not bool
    let bad_if = Expr::If(
        Box::new(Expr::Lit(1)),
        Box::new(Expr::Lit(2)),
        Box::new(Expr::Lit(3)),
    );
    assert!(type_check(&bad_if, &env).is_err());

    // Error: branches have different types
    let mismatch_if = Expr::If(
        Box::new(Expr::BoolLit(true)),
        Box::new(Expr::Lit(1)),
        Box::new(Expr::BoolLit(false)),
    );
    assert!(type_check(&mismatch_if, &env).is_err());
}

#[test]
fn let_and_var() {
    use std::collections::HashMap;

    let env = HashMap::new();
    // let x = 5 in x + 10
    let let_expr = Expr::Let(
        "x".into(),
        Box::new(Expr::Lit(5)),
        Box::new(Expr::Add(
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::Lit(10)),
        )),
    );
    assert_eq!(type_check(&let_expr, &env).unwrap(), Type::Int);

    // Error: undefined variable
    assert!(type_check(&Expr::Var("undefined".into()), &env).is_err());
}

#[test]
fn lambda_and_apply() {
    use std::collections::HashMap;

    let env = HashMap::new();
    // lambda (x: Int) -> x + 1
    let lambda = Expr::Lambda(
        vec![("x".into(), Type::Int)],
        Box::new(Expr::Add(
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::Lit(1)),
        )),
    );
    let ty = type_check(&lambda, &env).unwrap();
    assert_eq!(ty, Type::Fn(vec![Type::Int], Box::new(Type::Int)));

    // Apply: (lambda)(42)
    let apply = Expr::Apply(
        Box::new(lambda.clone()),
        vec![Expr::Lit(42)],
    );
    assert_eq!(type_check(&apply, &env).unwrap(), Type::Int);
}

#[test]
fn apply_type_errors() {
    use std::collections::HashMap;

    let env = HashMap::new();
    // Apply a non-function
    let bad_apply = Expr::Apply(
        Box::new(Expr::Lit(5)),
        vec![Expr::Lit(1)],
    );
    assert!(type_check(&bad_apply, &env).is_err());

    // Wrong argument type
    let lambda = Expr::Lambda(
        vec![("x".into(), Type::Int)],
        Box::new(Expr::Var("x".into())),
    );
    let wrong_arg = Expr::Apply(
        Box::new(lambda.clone()),
        vec![Expr::BoolLit(true)],
    );
    assert!(type_check(&wrong_arg, &env).is_err());

    // Wrong number of arguments
    let too_many = Expr::Apply(
        Box::new(lambda),
        vec![Expr::Lit(1), Expr::Lit(2)],
    );
    assert!(type_check(&too_many, &env).is_err());
}

#[test]
fn higher_order_function() {
    use std::collections::HashMap;

    let env = HashMap::new();
    // let apply_fn = lambda (f: Fn([Int], Int), x: Int) -> f(x)
    // in apply_fn(lambda (n: Int) -> n + n, 5)
    let inner_lambda = Expr::Lambda(
        vec![("n".into(), Type::Int)],
        Box::new(Expr::Add(
            Box::new(Expr::Var("n".into())),
            Box::new(Expr::Var("n".into())),
        )),
    );
    let fn_type = Type::Fn(vec![Type::Int], Box::new(Type::Int));
    let apply_fn = Expr::Lambda(
        vec![("f".into(), fn_type.clone()), ("x".into(), Type::Int)],
        Box::new(Expr::Apply(
            Box::new(Expr::Var("f".into())),
            vec![Expr::Var("x".into())],
        )),
    );
    let full_expr = Expr::Let(
        "apply_fn".into(),
        Box::new(apply_fn),
        Box::new(Expr::Apply(
            Box::new(Expr::Var("apply_fn".into())),
            vec![inner_lambda, Expr::Lit(5)],
        )),
    );
    assert_eq!(type_check(&full_expr, &env).unwrap(), Type::Int);
}

#[test]
fn variable_shadowing() {
    use std::collections::HashMap;

    let env = HashMap::new();
    // let x = true in (let x = 5 in x + 1)
    // inner x should shadow outer x (Bool -> Int)
    let expr = Expr::Let(
        "x".into(),
        Box::new(Expr::BoolLit(true)),
        Box::new(Expr::Let(
            "x".into(),
            Box::new(Expr::Lit(5)),
            Box::new(Expr::Add(
                Box::new(Expr::Var("x".into())),
                Box::new(Expr::Lit(1)),
            )),
        )),
    );
    assert_eq!(type_check(&expr, &env).unwrap(), Type::Int);
}
