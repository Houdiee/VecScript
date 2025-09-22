#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(f64),
    Identifier(String),
    Keyword(KeywordType),
    Operator(OperatorType),
    Delimiter(DelimiterType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DelimiterType {
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    LChevron, // <
    RChevron, // >
    Comma,    // ,
    Colon,    // :
    Pipe,     // |
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KeywordType {
    Solve,
    Let,
    In,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperatorType {
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Power,    // ^
    Dot,      // .
}
