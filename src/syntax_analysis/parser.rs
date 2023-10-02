use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: None,
        };

        parser.next();

        parser
    }

    pub fn parse(&mut self) -> JuvinilResult<()> {
        self.program()
    }

    fn next(&mut self) -> Option<&Token> {
        self.pos += 1;

        if self.pos >= self.tokens.len() as i32 {
            self.pos = (self.tokens.len() - 1) as i32;
        }

        let res = self.tokens.get(self.pos as usize);

        if let Some(next) = res {
            tracing::info!("Next: {:?}", next);
        }

        self.current_token = Some(res.unwrap().clone());

        res
    }

    fn consume(&mut self, token_type: TokenType, value: Option<&str>) -> JuvinilResult<()> {
        let ref_self = self
            .current_token
            .as_ref()
            .ok_or(JuvinilError::ParsingError)?;

        if ref_self.token_type != token_type {
            return Err(JuvinilError::SyntaxError);
        }

        if value.is_some() && ref_self.value.to_owned() != value.unwrap() {
            return Err(JuvinilError::SyntaxError);
        }

        self.next();

        Ok(())
    }

    fn program(&mut self) -> JuvinilResult<()> {
        self.asgn()
    }

    fn decl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;
        self.consume(TokenType::ID, None)
    }

    fn asgn(&mut self) -> JuvinilResult<()> {
        self.decl()?;
        self.consume(TokenType::OPERATOR, Some("="))
    }

    fn _endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))
    }
}
