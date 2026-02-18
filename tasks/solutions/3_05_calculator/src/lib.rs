pub fn calculate(expr: &str) -> f64 {
    let tokens = tokenize(expr);
    let mut pos = 0;
    parse_expr(&tokens, &mut pos)
}

#[derive(Debug, Clone)]
enum Token {
    Num(f64),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' => { i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            '+' | '-' | '*' | '/' => { tokens.push(Token::Op(chars[i])); i += 1; }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num_str = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Num(num_str.parse().unwrap()));
            }
            _ => { i += 1; }
        }
    }
    tokens
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> f64 {
    let mut left = parse_term(tokens, pos);
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Op('+') => { *pos += 1; left += parse_term(tokens, pos); }
            Token::Op('-') => { *pos += 1; left -= parse_term(tokens, pos); }
            _ => break,
        }
    }
    left
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> f64 {
    let mut left = parse_factor(tokens, pos);
    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Op('*') => { *pos += 1; left *= parse_factor(tokens, pos); }
            Token::Op('/') => { *pos += 1; left /= parse_factor(tokens, pos); }
            _ => break,
        }
    }
    left
}

fn parse_factor(tokens: &[Token], pos: &mut usize) -> f64 {
    if *pos < tokens.len() {
        if let Token::Op('-') = &tokens[*pos] {
            *pos += 1;
            return -parse_factor(tokens, pos);
        }
    }
    match &tokens[*pos] {
        Token::Num(n) => { let v = *n; *pos += 1; v }
        Token::LParen => {
            *pos += 1;
            let v = parse_expr(tokens, pos);
            *pos += 1; // RParen
            v
        }
        _ => 0.0,
    }
}
