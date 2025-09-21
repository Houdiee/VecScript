#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum TokenType {
    Number(f64),
    Identifier(String),
    Keyword(KeywordType),
    Operator(OperatorType),
    Delimiter(DelimiterType),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum KeywordType {
    Solve,
    Let,
    In,
}

#[derive(Debug)]
pub enum OperatorType {
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Power,    // ^
    Dot,      // .
}
