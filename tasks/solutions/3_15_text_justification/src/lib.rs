pub fn justify(words: &[&str], max_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < words.len() {
        let mut line_len = words[i].len();
        let mut j = i + 1;
        while j < words.len() && line_len + 1 + words[j].len() <= max_width {
            line_len += 1 + words[j].len();
            j += 1;
        }

        let num_words = j - i;
        let is_last_line = j == words.len();

        if num_words == 1 || is_last_line {
            // Left justify
            let mut line = words[i..j].join(" ");
            line.push_str(&" ".repeat(max_width - line.len()));
            result.push(line);
        } else {
            let total_chars: usize = words[i..j].iter().map(|w| w.len()).sum();
            let total_spaces = max_width - total_chars;
            let gaps = num_words - 1;
            let base_spaces = total_spaces / gaps;
            let extra = total_spaces % gaps;

            let mut line = String::new();
            for k in 0..num_words {
                line.push_str(words[i + k]);
                if k < gaps {
                    let spaces = base_spaces + if k < extra { 1 } else { 0 };
                    line.push_str(&" ".repeat(spaces));
                }
            }
            result.push(line);
        }

        i = j;
    }

    result
}
