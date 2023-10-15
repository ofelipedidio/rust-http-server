use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum EscapeError {
    InvalidEscapeCharacter(char),
    IncompleteEscapeSequence,
}

impl Display for EscapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EscapeError::InvalidEscapeCharacter(c) => write!(
                f,
                "Expected a HEX character, found {c:?} while decoding a %-encoded character"
            ),
            EscapeError::IncompleteEscapeSequence => write!(
                f,
                "Expected a  HEX character, found EOF while decoding a %-encoded character"
            ),
        }
    }
}

impl Error for EscapeError {}

pub fn unescape_str(seg: &str) -> Result<String, Box<dyn Error>> {
    let mut a = seg.chars();

    let mut line = String::new();

    loop {
        let c = match a.next() {
            Some(c) => c,
            None => break,
        };

        let c = match c {
            '%' => {
                let first = match a.next() {
                    Some(first @ '0'..='9') => (first as u8) - ('0' as u8),
                    Some(first @ 'a'..='z') => (first as u8) - ('a' as u8) + 10,
                    Some(first @ 'A'..='Z') => (first as u8) - ('A' as u8) + 10,
                    Some(first) => {
                        return Err(Box::new(EscapeError::InvalidEscapeCharacter(first.clone())))
                    }
                    None => return Err(Box::new(EscapeError::IncompleteEscapeSequence)),
                };
                let second = match a.next() {
                    Some(second @ '0'..='9') => (second as u8) - ('0' as u8),
                    Some(second @ 'a'..='z') => (second as u8) - ('a' as u8) + 10,
                    Some(second @ 'A'..='Z') => (second as u8) - ('A' as u8) + 10,
                    Some(second) => {
                        return Err(Box::new(EscapeError::InvalidEscapeCharacter(
                            second.clone(),
                        )))
                    }
                    None => return Err(Box::new(EscapeError::IncompleteEscapeSequence)),
                };
                let val = (first << 4) | second; // Compute the byte
                val as char
            }
            c => c,
        };

        line.push(c);
    }

    Ok(line)
}


