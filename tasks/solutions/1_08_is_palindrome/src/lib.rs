pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    true
}
