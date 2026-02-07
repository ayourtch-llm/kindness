pub fn is_anagram(s: &str, t: &str) -> bool {
    let mut counts = [0i32; 26];
    for c in s.chars().filter(|c| c.is_alphabetic()) {
        counts[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
    }
    for c in t.chars().filter(|c| c.is_alphabetic()) {
        counts[(c.to_ascii_lowercase() as u8 - b'a') as usize] -= 1;
    }
    counts.iter().all(|&x| x == 0)
}
