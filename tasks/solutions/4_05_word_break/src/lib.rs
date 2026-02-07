pub fn word_break(s: &str, dict: &[&str]) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for &word in dict {
            let wlen = word.len();
            if wlen <= i && dp[i - wlen] && &s[i - wlen..i] == word {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}
