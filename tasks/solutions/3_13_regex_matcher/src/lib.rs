pub fn is_match(text: &str, pattern: &str) -> bool {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let mut dp = vec![vec![false; p.len() + 1]; t.len() + 1];
    dp[0][0] = true;

    // Handle patterns like a*, a*b*, a*b*c* that can match empty string
    for j in 1..=p.len() {
        if p[j - 1] == '*' && j >= 2 {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=t.len() {
        for j in 1..=p.len() {
            if p[j - 1] == '*' {
                // Zero occurrences of preceding element
                dp[i][j] = dp[i][j - 2];
                // One or more occurrences
                if p[j - 2] == '.' || p[j - 2] == t[i - 1] {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if p[j - 1] == '.' || p[j - 1] == t[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[t.len()][p.len()]
}
