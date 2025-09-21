use crate::lexer::token::OperatorType;

#[derive(Debug)]
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
    Asin(Box<Expression>),
    Acos(Box<Expression>),
    Atan(Box<Expression>),

    BinaryOp {
        left: Box<Expression>,
        operator: OperatorType,
        right: Box<Expression>,
    },

    // global functions
    Mag(Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    Cross(Box<Expression>, Box<Expression>),

    MethodCall {
        object: Box<Expression>,
        method: String,
    },
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    LetIn {
        variable: String,
        value: Box<Expression>,
        body: Box<Statement>,
    },
    SolveIn {
        variable: String,
        equation: Box<Expression>,
    },
}
