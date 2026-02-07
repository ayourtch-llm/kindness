pub struct Parser<T> {
    pub parse: Box<dyn Fn(&[u8]) -> Result<(T, &[u8]), String>>,
}

pub fn u8_parser() -> Parser<u8> {
    Parser {
        parse: Box::new(|input: &[u8]| {
            if input.is_empty() {
                Err("unexpected end of input".to_string())
            } else {
                Ok((input[0], &input[1..]))
            }
        }),
    }
}

pub fn u16_be() -> Parser<u16> {
    Parser {
        parse: Box::new(|input: &[u8]| {
            if input.len() < 2 {
                Err("need 2 bytes for u16".to_string())
            } else {
                let val = u16::from_be_bytes([input[0], input[1]]);
                Ok((val, &input[2..]))
            }
        }),
    }
}

pub fn u32_be() -> Parser<u32> {
    Parser {
        parse: Box::new(|input: &[u8]| {
            if input.len() < 4 {
                Err("need 4 bytes for u32".to_string())
            } else {
                let val = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
                Ok((val, &input[4..]))
            }
        }),
    }
}

pub fn bytes(n: usize) -> Parser<Vec<u8>> {
    Parser {
        parse: Box::new(move |input: &[u8]| {
            if input.len() < n {
                Err(format!("need {} bytes, got {}", n, input.len()))
            } else {
                Ok((input[..n].to_vec(), &input[n..]))
            }
        }),
    }
}

impl<T: 'static> Parser<T> {
    pub fn map<U: 'static>(self, f: impl Fn(T) -> U + 'static) -> Parser<U> {
        Parser {
            parse: Box::new(move |input: &[u8]| {
                let (val, rest) = (self.parse)(input)?;
                Ok((f(val), rest))
            }),
        }
    }

    pub fn then<U: 'static>(self, next: Parser<U>) -> Parser<(T, U)> {
        Parser {
            parse: Box::new(move |input: &[u8]| {
                let (val1, rest1) = (self.parse)(input)?;
                let (val2, rest2) = (next.parse)(rest1)?;
                Ok(((val1, val2), rest2))
            }),
        }
    }

    pub fn repeat(self, count: usize) -> Parser<Vec<T>> {
        Parser {
            parse: Box::new(move |input: &[u8]| {
                let mut results = Vec::with_capacity(count);
                let mut remaining = input;
                for _ in 0..count {
                    let (val, rest) = (self.parse)(remaining)?;
                    results.push(val);
                    remaining = rest;
                }
                Ok((results, remaining))
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TlvRecord {
    pub tag: u8,
    pub value: Vec<u8>,
}

pub fn tlv_parser() -> Parser<TlvRecord> {
    Parser {
        parse: Box::new(|input: &[u8]| {
            let (tag, rest) = (u8_parser().parse)(input)?;
            let (length, rest) = (u16_be().parse)(rest)?;
            let (value, rest) = (bytes(length as usize).parse)(rest)?;
            Ok((TlvRecord { tag, value }, rest))
        }),
    }
}

pub fn parse_tlv_stream(data: &[u8]) -> Result<Vec<TlvRecord>, String> {
    let parser = tlv_parser();
    let mut records = Vec::new();
    let mut remaining = data;
    while !remaining.is_empty() {
        let (record, rest) = (parser.parse)(remaining)?;
        records.push(record);
        remaining = rest;
    }
    Ok(records)
}
