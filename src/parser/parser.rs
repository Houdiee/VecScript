use crate::lexer::{error::LexerError, lexer::Lexer, token::Token};

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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_index)
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
