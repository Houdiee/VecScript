use std::fmt;

pub enum LexerError {
    UnexpectedCharacter {
        line: u32,
        column: u32,
        character: char,
    },
    InvalidNumber {
        line: u32,
        column: u32,
        literal: String,
    },
    InvalidDelimiter {
        line: u32,
        column: u32,
        delimiter: char,
    },
    InvalidOperator {
        line: u32,
        column: u32,
        operator: char,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter {
                line,
                column,
                character,
            } => {
                write!(
                    f,
                    "Lexical Error: Unexpected character '{character}' at line {line}, column {column}",
                )
            }

            LexerError::InvalidNumber {
                line,
                column,
                literal,
            } => {
                write!(
                    f,
                    "Lexical Error: Invalid number '{literal}' at line {line}, column {column}"
                )
            }

            LexerError::InvalidDelimiter {
                line,
                column,
                delimiter,
            } => write!(
                f,
                "Lexical Error: Invalid number '{delimiter}' at line {line}, column {column}"
            ),

            LexerError::InvalidOperator {
                line,
                column,
                operator,
            } => write!(
                f,
                "Lexical Error: Invalid number '{operator}' at line {line}, column {column}"
            ),
        }
    }
}
