#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;
    let result = parse_value(&chars, &mut pos)?;
    skip_whitespace(&chars, &mut pos);
    if pos != chars.len() {
        return Err(format!("Unexpected trailing characters at position {}", pos));
    }
    Ok(result)
}

fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn parse_value(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
        return Err("Unexpected end of input".to_string());
    }
    match chars[*pos] {
        'n' => parse_null(chars, pos),
        't' | 'f' => parse_bool(chars, pos),
        '"' => parse_string(chars, pos).map(JsonValue::Str),
        '[' => parse_array(chars, pos),
        '{' => parse_object(chars, pos),
        '-' | '0'..='9' => parse_number(chars, pos),
        c => Err(format!("Unexpected character '{}' at position {}", c, pos)),
    }
}

fn parse_null(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    if chars[*pos..].iter().take(4).collect::<String>() == "null" {
        *pos += 4;
        Ok(JsonValue::Null)
    } else {
        Err(format!("Expected 'null' at position {}", pos))
    }
}

fn parse_bool(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    if chars[*pos..].iter().take(4).collect::<String>() == "true" {
        *pos += 4;
        Ok(JsonValue::Bool(true))
    } else if chars[*pos..].iter().take(5).collect::<String>() == "false" {
        *pos += 5;
        Ok(JsonValue::Bool(false))
    } else {
        Err(format!("Expected boolean at position {}", pos))
    }
}

fn parse_string(chars: &[char], pos: &mut usize) -> Result<String, String> {
    if chars[*pos] != '"' {
        return Err(format!("Expected '\"' at position {}", pos));
    }
    *pos += 1;
    let mut s = String::new();
    while *pos < chars.len() && chars[*pos] != '"' {
        s.push(chars[*pos]);
        *pos += 1;
    }
    if *pos >= chars.len() {
        return Err("Unterminated string".to_string());
    }
    *pos += 1; // closing quote
    Ok(s)
}

fn parse_number(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    let start = *pos;
    if chars[*pos] == '-' {
        *pos += 1;
    }
    while *pos < chars.len() && chars[*pos].is_ascii_digit() {
        *pos += 1;
    }
    if *pos < chars.len() && chars[*pos] == '.' {
        *pos += 1;
        while *pos < chars.len() && chars[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
    let num_str: String = chars[start..*pos].iter().collect();
    num_str
        .parse::<f64>()
        .map(JsonValue::Number)
        .map_err(|e| format!("Invalid number: {}", e))
}

fn parse_array(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    *pos += 1; // '['
    skip_whitespace(chars, pos);
    let mut arr = Vec::new();
    if *pos < chars.len() && chars[*pos] == ']' {
        *pos += 1;
        return Ok(JsonValue::Array(arr));
    }
    loop {
        let val = parse_value(chars, pos)?;
        arr.push(val);
        skip_whitespace(chars, pos);
        if *pos < chars.len() && chars[*pos] == ',' {
            *pos += 1;
        } else {
            break;
        }
    }
    if *pos < chars.len() && chars[*pos] == ']' {
        *pos += 1;
        Ok(JsonValue::Array(arr))
    } else {
        Err("Expected ']'".to_string())
    }
}

fn parse_object(chars: &[char], pos: &mut usize) -> Result<JsonValue, String> {
    *pos += 1; // '{'
    skip_whitespace(chars, pos);
    let mut obj = Vec::new();
    if *pos < chars.len() && chars[*pos] == '}' {
        *pos += 1;
        return Ok(JsonValue::Object(obj));
    }
    loop {
        skip_whitespace(chars, pos);
        if *pos >= chars.len() || chars[*pos] != '"' {
            return Err(format!("Expected string key at position {}", pos));
        }
        let key = parse_string(chars, pos)?;
        skip_whitespace(chars, pos);
        if *pos >= chars.len() || chars[*pos] != ':' {
            return Err("Expected ':'".to_string());
        }
        *pos += 1;
        let val = parse_value(chars, pos)?;
        obj.push((key, val));
        skip_whitespace(chars, pos);
        if *pos < chars.len() && chars[*pos] == ',' {
            *pos += 1;
        } else {
            break;
        }
    }
    if *pos < chars.len() && chars[*pos] == '}' {
        *pos += 1;
        Ok(JsonValue::Object(obj))
    } else {
        Err("Expected '}'".to_string())
    }
}
