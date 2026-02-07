use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    Fn(Vec<Type>, Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i64),
    BoolLit(bool),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Lambda(Vec<(String, Type)>, Box<Expr>),
    Apply(Box<Expr>, Vec<Expr>),
}

pub fn type_check(expr: &Expr, env: &HashMap<String, Type>) -> Result<Type, String> {
    match expr {
        Expr::Lit(_) => Ok(Type::Int),
        Expr::BoolLit(_) => Ok(Type::Bool),
        Expr::Var(name) => {
            env.get(name).cloned().ok_or_else(|| format!("undefined variable: {}", name))
        }
        Expr::Add(a, b) => {
            let ta = type_check(a, env)?;
            let tb = type_check(b, env)?;
            if ta != Type::Int {
                return Err("Add: left operand must be Int".to_string());
            }
            if tb != Type::Int {
                return Err("Add: right operand must be Int".to_string());
            }
            Ok(Type::Int)
        }
        Expr::Eq(a, b) => {
            let ta = type_check(a, env)?;
            let tb = type_check(b, env)?;
            if ta != tb {
                return Err("Eq: operands must have the same type".to_string());
            }
            match &ta {
                Type::Int | Type::Bool => {}
                _ => return Err("Eq: operands must be Int or Bool".to_string()),
            }
            Ok(Type::Bool)
        }
        Expr::If(cond, then_expr, else_expr) => {
            let tc = type_check(cond, env)?;
            if tc != Type::Bool {
                return Err("If: condition must be Bool".to_string());
            }
            let tt = type_check(then_expr, env)?;
            let te = type_check(else_expr, env)?;
            if tt != te {
                return Err("If: branches must have the same type".to_string());
            }
            Ok(tt)
        }
        Expr::Let(name, value, body) => {
            let tv = type_check(value, env)?;
            let mut new_env = env.clone();
            new_env.insert(name.clone(), tv);
            type_check(body, &new_env)
        }
        Expr::Lambda(params, body) => {
            let mut new_env = env.clone();
            let param_types: Vec<Type> = params.iter().map(|(_, t)| t.clone()).collect();
            for (name, ty) in params {
                new_env.insert(name.clone(), ty.clone());
            }
            let ret_type = type_check(body, &new_env)?;
            Ok(Type::Fn(param_types, Box::new(ret_type)))
        }
        Expr::Apply(func, args) => {
            let tf = type_check(func, env)?;
            match tf {
                Type::Fn(param_types, ret_type) => {
                    if args.len() != param_types.len() {
                        return Err(format!(
                            "Apply: expected {} arguments, got {}",
                            param_types.len(),
                            args.len()
                        ));
                    }
                    for (i, (arg, expected)) in args.iter().zip(param_types.iter()).enumerate() {
                        let ta = type_check(arg, env)?;
                        if ta != *expected {
                            return Err(format!(
                                "Apply: argument {} has type {:?}, expected {:?}",
                                i, ta, expected
                            ));
                        }
                    }
                    Ok(*ret_type)
                }
                _ => Err("Apply: callee is not a function".to_string()),
            }
        }
    }
}
