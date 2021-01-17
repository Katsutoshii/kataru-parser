use super::{Parsable, ParseError};

#[derive(Debug)]
pub enum Operator {
    ADD,
    SUB,
    SET,
}

impl Parsable<'_> for Operator {
    fn parse(op: &str) -> Result<Self, ParseError> {
        match op {
            "+=" => Ok(Self::ADD),
            "-=" => Ok(Self::SUB),
            "=" => Ok(Self::SET),
            _ => Err(perror!("No valid Operator matches {}", op)),
        }
    }
}
