#[derive(Debug)]
pub enum Token<'a> {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Dup,
    Drop,
    Swap,
    Over,
    Colon,
    Word(&'a str),
    Semicolon,
    End,
    Value(i32),
}

use Token::*;

pub struct Tokenizer<'a> {
    source: &'a str,
}

pub struct TokenIter<'a> {
    position: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Self { source }
    }

    pub fn iter(&self) -> TokenIter<'a> {
        TokenIter {
            position: Some(self.source),
        }
    }
}

impl<'a> std::iter::Iterator for TokenIter<'a> {
    type Item = Result<Token<'a>, super::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match next_token(self.position?) {
            Ok((End, _position)) => {
                self.position = None;
                Some(Ok(End))
            }
            Ok((token, position)) => {
                self.position = Some(position);
                Some(Ok(token))
            }
            Err(()) => {
                self.position = None;
                Some(Err(super::Error::InvalidWord))
            }
        }
    }
}

macro_rules! whitespace {
    () => {
        b' ' | b'\t' | b'\r' | b'\n'
    };
}

macro_rules! digit {
    () => {
        b'0'..=b'9'
    };
}

macro_rules! word {
  () => {
    b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'-'
  };
}

fn next_token(string: &str) -> Result<(Token, &str), ()> {
    let bytes = string.as_bytes();
    if string.len() == 0 {
        return Ok((End, string));
    }
    match bytes[0] {
        whitespace!() => next_token(&string[1..]),
        digit!() => parse_value(string),
        b'+' => Ok((Plus, &string[1..])),
        b'-' => {
            if bytes.len() > 1 {
                match bytes[1] {
                    whitespace!() => Ok((Minus, &string[1..])),
                    _ => parse_value(string),
                }
            } else {
                Ok((Minus, &string[1..]))
            }
        }
        b'*' => Ok((Asterisk, &string[1..])),
        b'/' => Ok((Slash, &string[1..])),
        b':' => Ok((Colon, &string[1..])),
        b';' => Ok((Semicolon, &string[1..])),
        b'd' | b'D' => match bytes[1] {
            b'u' | b'U' => match bytes[2] {
                b'p' | b'P' => Ok((Dup, &string[3..])),
                _ => parse_word(string),
            },
            b'r' | b'R' => match bytes[2] {
                b'o' | b'O' => match bytes[3] {
                    b'p' | b'P' => Ok((Drop, &string[4..])),
                    _ => parse_word(string),
                },
                _ => parse_word(string),
            },
            _ => parse_word(string),
        },
        b'o' | b'O' => match bytes[1] {
            b'v' | b'V' => match bytes[2] {
                b'e' | b'E' => match bytes[3] {
                    b'r' | b'R' => Ok((Over, &string[4..])),
                    _ => parse_word(string),
                },
                _ => parse_word(string),
            },
            _ => parse_word(string),
        },
        b's' | b'S' => match bytes[1] {
            b'w' | b'W' => match bytes[2] {
                b'a' | b'A' => match bytes[3] {
                    b'p' | b'P' => Ok((Swap, &string[4..])),
                    _ => parse_word(string),
                },
                _ => parse_word(string),
            },
            _ => parse_word(string),
        },
        _ => parse_word(string),
    }
}

fn parse_word(string: &str) -> Result<(Token, &str), ()> {
    let mut split_at = 0;
    while split_at < string.len() {
        match string.as_bytes()[split_at] {
            whitespace!() => break,
            word!() => split_at += 1,
            _ => return Err(()),
        }
    }
    if split_at == 0 {
        Err(())
    } else {
        Ok((Word(&string[0..split_at]), &string[split_at..]))
    }
}

fn parse_value(string: &str) -> Result<(Token, &str), ()> {
    let mut split_at = 0;
    if string.as_bytes()[0] == b'-' {
        split_at = 1;
    }
    while split_at < string.len() {
        match string.as_bytes()[split_at] {
            whitespace!() => break,
            digit!() => split_at += 1,
            _ => return Err(()),
        }
    }
    if split_at == 0 || string.as_bytes()[split_at - 1] == b'-' {
        Err(())
    } else {
        if let Ok(v) = string[0..split_at].parse() {
            Ok((Value(v), &string[split_at..]))
        } else {
            Err(())
        }
    }
}
