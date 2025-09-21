pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: u32,
    pub column: u32,
}

pub enum TokenType {
    Number(f64),
    Identifier(String),
    Keyword(KeywordType),
    Operator(OperatorType),
    Delimiter(DelimiterType),
}

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

pub enum KeywordType {
    Solve,
    Let,
    In,

    // FUNCTIONS
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Mag,
    Dot,
    Cross,

    // METHODS
    Rows,
    Cols,
}

pub enum OperatorType {
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Power,    // ^
    Dot,      // .
}
