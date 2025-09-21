use std::{iter::Peekable, str::Chars};

use crate::lexer::token::{DelimiterType, OperatorType, Token, TokenType};

pub struct Lexer<'a> {
    pub source: &'a str,
    pub chars: Peekable<Chars<'a>>,
    pub line: u32,
    pub column: u32,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        let current_char = self.chars.next()?;

        match current_char {
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            c => {
                self.column += 1;
            }
        }
        Some(current_char)
    }

    fn tokenize(&mut self) -> Option<Token> {
        let start_line = self.line;
        let start_column = self.column + 1;
        let current = self.advance()?;

        match current {
            operator @ ('=' | '+' | '-' | '*' | '/' | '^' | '.') => {
                Some(self.tokenize_operator(operator, start_line, start_column))
            }

            c if c.is_alphanumeric() || c == '_' => {
                // Either IDENTIFIER or KEYWORD
                todo!()
            }

            c if c.is_digit(10) => {
                /// PARSE THE NUMBERRRRRR
                todo!()
            }

            delimiter @ ('(' | ')' | '[' | ']' | '<' | '>' | ',' | ':' | '|') => {
                Some(self.tokenize_delimiter(delimiter, start_line, start_column))
            }

            _ => None,
        }
    }

    fn tokenize_delimiter(&self, delimiter: char, line: u32, column: u32) -> Token {
        let literal = delimiter.to_string();
        match delimiter {
            '(' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            ')' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            '[' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            ']' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            '<' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            '>' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            ',' => Token {
                token_type: TokenType::Delimiter(DelimiterType::LParen),
                literal,
                line,
                column,
            },
            ':' => Token {
                token_type: TokenType::Delimiter(DelimiterType::Colon),
                literal,
                line,
                column,
            },
            '|' => Token {
                token_type: TokenType::Delimiter(DelimiterType::Pipe),
                literal,
                line,
                column,
            },
            _ => panic!("Unknown delimiter `{delimiter}`"),
        }
    }

    fn tokenize_operator(&self, operator: char, line: u32, column: u32) -> Token {
        let literal = operator.to_string();
        match operator {
            '=' => Token {
                token_type: TokenType::Operator(OperatorType::Equals),
                literal,
                line,
                column,
            },
            '+' => Token {
                token_type: TokenType::Operator(OperatorType::Plus),
                literal,
                line,
                column,
            },
            '-' => Token {
                token_type: TokenType::Operator(OperatorType::Minus),
                literal,
                line,
                column,
            },
            '*' => Token {
                token_type: TokenType::Operator(OperatorType::Multiply),
                literal,
                line,
                column,
            },
            '/' => Token {
                token_type: TokenType::Operator(OperatorType::Divide),
                literal,
                line,
                column,
            },
            '^' => Token {
                token_type: TokenType::Operator(OperatorType::Power),
                literal,
                line,
                column,
            },
            '.' => Token {
                token_type: TokenType::Operator(OperatorType::Dot),
                literal,
                line,
                column,
            },
            _ => panic!("Unknown operator `{operator}`"),
        }
    }
}
