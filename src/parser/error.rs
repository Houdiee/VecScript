use std::fmt::Display;

pub enum ParserError {
    UnexpectedToken {
        expected: String,
        found: String,
        line: u32,
        column: u32,
    },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken {
                expected,
                found,
                line,
                column,
            } => write!(f, "Parser Error: Expected {expected} but found {found} at line {line}, column {column}"),
        }
    }
}
