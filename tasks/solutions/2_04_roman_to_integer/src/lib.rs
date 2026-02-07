pub fn roman_to_int(s: &str) -> i32 {
    let value = |c| match c {
        'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50,
        'C' => 100, 'D' => 500, 'M' => 1000, _ => 0,
    };
    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;
    for i in 0..chars.len() {
        let curr = value(chars[i]);
        let next = if i + 1 < chars.len() { value(chars[i + 1]) } else { 0 };
        if curr < next {
            result -= curr;
        } else {
            result += curr;
        }
    }
    result
}
