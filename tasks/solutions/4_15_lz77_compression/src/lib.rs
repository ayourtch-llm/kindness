#[derive(Debug, Clone, PartialEq)]
pub enum LzToken {
    Literal(u8),
    Match { offset: usize, length: usize },
}

pub fn compress(input: &[u8], window_size: usize) -> Vec<LzToken> {
    let mut tokens = Vec::new();
    let n = input.len();
    let mut i = 0;

    while i < n {
        let mut best_offset = 0;
        let mut best_length = 0;

        let search_start = if i > window_size { i - window_size } else { 0 };

        for j in search_start..i {
            let mut length = 0;
            while i + length < n && input[j + length] == input[i + length] {
                length += 1;
                if j + length >= i {
                    // Allow overlapping matches by wrapping
                    break;
                }
            }
            if length > best_length {
                best_length = length;
                best_offset = i - j;
            }
        }

        if best_length >= 3 {
            tokens.push(LzToken::Match {
                offset: best_offset,
                length: best_length,
            });
            i += best_length;
        } else {
            tokens.push(LzToken::Literal(input[i]));
            i += 1;
        }
    }

    tokens
}

pub fn decompress(tokens: &[LzToken]) -> Vec<u8> {
    let mut output = Vec::new();

    for token in tokens {
        match token {
            LzToken::Literal(b) => output.push(*b),
            LzToken::Match { offset, length } => {
                let start = output.len() - offset;
                for i in 0..*length {
                    let byte = output[start + i];
                    output.push(byte);
                }
            }
        }
    }

    output
}
