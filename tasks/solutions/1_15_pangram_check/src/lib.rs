pub fn is_pangram(s: &str) -> bool {
    let mut seen = [false; 26];
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            seen[(c.to_ascii_lowercase() as u8 - b'a') as usize] = true;
        }
    }
    seen.iter().all(|&v| v)
}
