use solution::*;

#[test]
fn test_freezing_point() {
    let result = celsius_to_fahrenheit(0.0);
    assert!((result - 32.0).abs() < 1e-9);
}

#[test]
fn test_boiling_point() {
    let result = celsius_to_fahrenheit(100.0);
    assert!((result - 212.0).abs() < 1e-9);
}

#[test]
fn test_negative() {
    let result = celsius_to_fahrenheit(-40.0);
    assert!((result - (-40.0)).abs() < 1e-9);
}

#[test]
fn test_body_temperature() {
    let result = celsius_to_fahrenheit(37.0);
    assert!((result - 98.6).abs() < 1e-9);
}
