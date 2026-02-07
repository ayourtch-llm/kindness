pub fn rle_encode(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let mut count = 1;
        while i + count < chars.len() && chars[i + count] == c {
            count += 1;
        }
        result.push(c);
        if count > 1 {
            result.push_str(&count.to_string());
        }
        i += count;
    }
    result
}
