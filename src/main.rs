mod global;
mod lexer;
mod vec;

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

pub enum Statement {
    ExpressionStatement(Expression),
    LetIn {
        variable: String,
        value: Box<Expression>,
        body: Box<Statement>,
    },
}

fn main() {
    println!("Hello, world!");
}
