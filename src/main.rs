mod global;
mod vec;

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
}

pub enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Dot,
}

pub enum Expression {
    // basic types
    Number(f64),
    Variable(String),
    Vector(Vec<Expression>),
    Matrix(Vec<Vec<Expression>>),

    // trig
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),

    // global functions
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Mag(Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    Cross(Box<Expression>, Box<Expression>),

    MethodCall {
        object: Box<Expression>,
        method: String,
    },
}

fn main() {
    println!("Hello, world!");
}
