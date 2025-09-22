use crate::{
    lexer::{
        error::LexerError,
        lexer::Lexer,
        token::{DelimiterType, KeywordType, OperatorType, Token, TokenType},
    },
    parser::{
        error::ParserError,
        statement::{Statement, Type},
    },
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    tokens: Vec<Token>,
    current_token_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self, LexerError> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token()? {
            tokens.push(token);
        }

        Ok(Self {
            lexer,
            tokens,
            current_token_index: 0,
        })
    }

    fn parse_program(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        while let Some(_) = self.peek() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let current_token = self.peek().ok_or_else(|| {
            let (line, column) = self.tokens.last().map(|t| (t.line, t.column)).unwrap_or((1, 1));
            ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: "EOF".to_string(),
                line,
                column,
            }
        })?;

        match current_token.token_type {
            TokenType::Keyword(KeywordType::Let) => self.parse_let_in_statement(),

            _ => Err(ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: current_token.literal.clone(),
                line: current_token.line,
                column: current_token.column,
            }),
        }
    }

    #[allow(unused)]
    fn parse_let_in_statement(&mut self) -> Result<Statement, ParserError> {
        let let_token = self.expect_and_advance("let", |token_type| matches!(token_type, TokenType::Keyword(KeywordType::Let)))?;
        let variable_token = self.expect_and_advance("a variable name", |token_type| matches!(token_type, TokenType::Identifier(_)))?;

        let variable_name = match &variable_token.token_type {
            TokenType::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        let mut variable_type = None;
        if let Some(token) = self.peek() {
            if let TokenType::Delimiter(DelimiterType::Colon) = token.token_type {
                variable_type = Some(self.parse_type()?);
            }
        }

        let equals_token = self.expect_and_advance("=", |token_type| matches!(token_type, TokenType::Operator(OperatorType::Equals)))?;
        // EXPRESSION TOKEN HERE
        let in_token = self.expect_and_advance("in", |token_type| matches!(token_type, TokenType::Keyword(KeywordType::In)))?;

        Ok(Statement::LetIn {
            variable: variable_name,
            variable_type: variable_type,
            value: todo!(),
            body: todo!(),
        })
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let type_token = self.expect_and_advance("a variable type", |token_type| matches!(token_type, TokenType::Identifier(_)))?;
        let parsed_type = match type_token.literal.as_str() {
            "num" => Type::Number,
            "bool" => Type::Bool,
            "sym" => Type::Symbolic,
            "vec" => Type::Vector(Box::new(self.parse_type()?)),
            "mtrx" => Type::Matrix(Box::new(self.parse_type()?)),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "a valid type".to_string(),
                    found: type_token.literal.clone(),
                    line: type_token.line,
                    column: type_token.column,
                });
            }
        };
        Ok(parsed_type)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_index)
    }

    fn expect_and_advance(&mut self, expected: &str, predicate: impl Fn(&TokenType) -> bool) -> Result<&Token, ParserError> {
        let (last_line, last_column) = self.tokens.last().map(|t| (t.line, t.column)).unwrap_or((1, 1));
        let current_token = self.advance().ok_or_else(|| ParserError::UnexpectedToken {
            expected: expected.to_string(),
            found: "EOF".to_string(),
            line: last_line,
            column: last_column,
        })?;

        if !predicate(&current_token.token_type) {
            return Err(ParserError::UnexpectedToken {
                expected: expected.to_string(),
                found: current_token.literal.clone(),
                line: current_token.line,
                column: current_token.column,
            });
        }
        Ok(current_token)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.current_token_index < self.tokens.len() {
            let token = &self.tokens[self.current_token_index];
            self.current_token_index += 1;
            Some(token)
        } else {
            None
        }
    }
}
