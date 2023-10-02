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

    fn verify_current(&mut self, token_type: TokenType, value: Option<&str>) -> bool {
        if let Some(ct) = self.current_token.clone() {
            if value.is_none() {
                return ct.token_type == token_type;
            }

            return ct.token_type == token_type && ct.value.as_str() == value.unwrap();
        }

        return false;
    }

    fn program(&mut self) -> JuvinilResult<()> {
        if self.verify_current(TokenType::SYMBOL, Some("{")) {
            return self.block();
        }

        if self.verify_current(TokenType::TYPE, None) {
            return self.decls();
        }

        self.stmts()
    }

    fn block(&mut self) -> JuvinilResult<()> {
        Ok(())
    }

    fn decls(&mut self) -> JuvinilResult<()> {
        Ok(())
    }

    fn stmts(&mut self) -> JuvinilResult<()> {
        Ok(())
    }

    fn _decl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;
        self.consume(TokenType::ID, None)
    }

    fn _asgn(&mut self) -> JuvinilResult<()> {
        self._decl()?;
        self.consume(TokenType::OPERATOR, Some("="))
    }

    fn _endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))
    }
}
