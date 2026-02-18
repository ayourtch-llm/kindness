use solution::*;

#[test]
fn test_parse_null() {
    assert_eq!(parse_json("null").unwrap(), JsonValue::Null);
}

#[test]
fn test_parse_bool() {
    assert_eq!(parse_json("true").unwrap(), JsonValue::Bool(true));
    assert_eq!(parse_json("false").unwrap(), JsonValue::Bool(false));
}

#[test]
fn test_parse_number() {
    if let JsonValue::Number(n) = parse_json("42").unwrap() {
        assert!((n - 42.0).abs() < 1e-9);
    } else {
        panic!("Expected Number");
    }
    if let JsonValue::Number(n) = parse_json("3.14").unwrap() {
        assert!((n - 3.14).abs() < 1e-9);
    } else {
        panic!("Expected Number");
    }
}

#[test]
fn test_parse_string() {
    assert_eq!(parse_json("\"hello\"").unwrap(), JsonValue::Str("hello".to_string()));
}

#[test]
fn test_parse_array() {
    let result = parse_json("[1, 2, 3]").unwrap();
    if let JsonValue::Array(arr) = result {
        assert_eq!(arr.len(), 3);
    } else {
        panic!("Expected Array");
    }
}

#[test]
fn test_parse_object() {
    let result = parse_json("{\"key\": \"value\", \"num\": 42}").unwrap();
    if let JsonValue::Object(obj) = result {
        assert_eq!(obj.len(), 2);
        assert_eq!(obj[0].0, "key");
        assert_eq!(obj[0].1, JsonValue::Str("value".to_string()));
        assert_eq!(obj[1].0, "num");
        if let JsonValue::Number(n) = obj[1].1 {
            assert!((n - 42.0).abs() < 1e-9);
        } else {
            panic!("Expected Number for 'num' key");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_nested() {
    let result = parse_json("{\"a\": [1, {\"b\": 2}]}").unwrap();
    if let JsonValue::Object(obj) = result {
        assert_eq!(obj[0].0, "a");
        if let JsonValue::Array(arr) = &obj[0].1 {
            assert_eq!(arr.len(), 2);
            if let JsonValue::Object(inner) = &arr[1] {
                assert_eq!(inner[0].0, "b");
            } else {
                panic!("Expected nested Object");
            }
        } else {
            panic!("Expected Array");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_empty_structures() {
    let arr = parse_json("[]").unwrap();
    assert_eq!(arr, JsonValue::Array(vec![]));
    let obj = parse_json("{}").unwrap();
    assert_eq!(obj, JsonValue::Object(vec![]));
}

#[test]
fn test_parse_invalid() {
    assert!(parse_json("undefined").is_err());
    assert!(parse_json("{missing_quotes: 1}").is_err());
}
