use std::collections::HashMap;

pub struct Evaluator {
    variables: HashMap<String, f64>,
    functions: HashMap<String, Box<dyn Fn(&[f64]) -> f64>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, val: f64) {
        self.variables.insert(name.to_string(), val);
    }

    pub fn register_function(&mut self, name: &str, f: Box<dyn Fn(&[f64]) -> f64>) {
        self.functions.insert(name.to_string(), f);
    }

    pub fn evaluate(&self, expr: &str) -> Result<f64, String> {
        let tokens = tokenize(expr)?;
        let mut pos = 0;
        let result = self.parse_add_sub(&tokens, &mut pos)?;
        if pos != tokens.len() {
            return Err(format!("Unexpected token at position {}", pos));
        }
        Ok(result)
    }

    fn parse_add_sub(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_mul_div(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    let right = self.parse_mul_div(tokens, pos)?;
                    left += right;
                }
                Token::Minus => {
                    *pos += 1;
                    let right = self.parse_mul_div(tokens, pos)?;
                    left -= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_exponent(tokens, pos)?;
        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Star => {
                    *pos += 1;
                    let right = self.parse_exponent(tokens, pos)?;
                    left *= right;
                }
                Token::Slash => {
                    *pos += 1;
                    let right = self.parse_exponent(tokens, pos)?;
                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    left /= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_exponent(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let base = self.parse_unary(tokens, pos)?;
        if *pos < tokens.len() && matches!(&tokens[*pos], Token::Caret) {
            *pos += 1;
            // Right-associative: recurse into parse_exponent
            let exp = self.parse_exponent(tokens, pos)?;
            Ok(base.powf(exp))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        if *pos < tokens.len() && matches!(&tokens[*pos], Token::Minus) {
            *pos += 1;
            let val = self.parse_unary(tokens, pos)?;
            Ok(-val)
        } else {
            self.parse_primary(tokens, pos)
        }
    }

    fn parse_primary(&self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }
        match &tokens[*pos] {
            Token::Number(n) => {
                let val = *n;
                *pos += 1;
                Ok(val)
            }
            Token::Ident(name) => {
                let name = name.clone();
                *pos += 1;
                // Check if it's a function call
                if *pos < tokens.len() && matches!(&tokens[*pos], Token::LParen) {
                    *pos += 1; // consume '('
                    let mut args = Vec::new();
                    if *pos < tokens.len() && !matches!(&tokens[*pos], Token::RParen) {
                        args.push(self.parse_add_sub(tokens, pos)?);
                        while *pos < tokens.len() && matches!(&tokens[*pos], Token::Comma) {
                            *pos += 1;
                            args.push(self.parse_add_sub(tokens, pos)?);
                        }
                    }
                    if *pos >= tokens.len() || !matches!(&tokens[*pos], Token::RParen) {
                        return Err("Expected ')'".to_string());
                    }
                    *pos += 1; // consume ')'
                    let func = self
                        .functions
                        .get(&name)
                        .ok_or_else(|| format!("Unknown function: {}", name))?;
                    Ok(func(&args))
                } else {
                    // Variable
                    self.variables
                        .get(&name)
                        .copied()
                        .ok_or_else(|| format!("Unknown variable: {}", name))
                }
            }
            Token::LParen => {
                *pos += 1;
                let val = self.parse_add_sub(tokens, pos)?;
                if *pos >= tokens.len() || !matches!(&tokens[*pos], Token::RParen) {
                    return Err("Expected ')'".to_string());
                }
                *pos += 1;
                Ok(val)
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '^' => { tokens.push(Token::Caret); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num: f64 = num_str
                    .parse()
                    .map_err(|_| format!("Invalid number: {}", num_str))?;
                tokens.push(Token::Number(num));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();
                tokens.push(Token::Ident(ident));
            }
            c => return Err(format!("Unexpected character: {}", c)),
        }
    }
    Ok(tokens)
}
