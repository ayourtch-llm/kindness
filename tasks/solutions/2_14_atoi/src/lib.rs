pub fn my_atoi(s: &str) -> i32 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }
    let mut chars = s.chars().peekable();
    let sign: i64 = match chars.peek() {
        Some('+') => { chars.next(); 1 }
        Some('-') => { chars.next(); -1 }
        _ => 1,
    };
    let mut result: i64 = 0;
    for c in chars {
        if let Some(d) = c.to_digit(10) {
            result = result * 10 + d as i64;
            if sign * result > i32::MAX as i64 {
                return i32::MAX;
            }
            if sign * result < i32::MIN as i64 {
                return i32::MIN;
            }
        } else {
            break;
        }
    }
    (sign * result) as i32
}
