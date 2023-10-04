use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

use super::parser_map::ParserMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Option<Token>,
    _parser_map: ParserMap,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: None,
            _parser_map: ParserMap::new(),
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

    fn get_current_values(&self) -> JuvinilResult<(TokenType, &str)> {
        if let None = self.current_token {
            return Err(JuvinilError::ParsingError);
        }

        let values = self.current_token.as_ref().unwrap().values();

        Ok(values)
    }

    fn program(&mut self) -> JuvinilResult<()> {
        match self.get_current_values()? {
            (TokenType::SYMBOL, "{") => return self.block(),
            (TokenType::TYPE, ..) => return self.decls(),
            _ => return self.decls(),
        }
    }

    fn block(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some("{"))?;
        self.decls()?;
        self.stmts()?;
        self.consume(TokenType::SYMBOL, Some("}"))
    }

    fn decls(&mut self) -> JuvinilResult<()> {
        while self.get_current_values()?.0 == TokenType::TYPE {
            self.decl()?;
        }

        Ok(())
    }

    fn decl(&mut self) -> JuvinilResult<()> {
        self.jvtype()?;
        self.consume(TokenType::ID, None)?;
        self.endexpr()
    }

    fn stmts(&mut self) -> JuvinilResult<()> {
        self.stmt()
    }

    fn stmt(&mut self) -> JuvinilResult<()> {
        match self.get_current_values()? {
            (TokenType::ID, ..) => self.asgn()?,
            _ => (),
        }

        Ok(())
    }

    fn jvtype(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;

        match self.get_current_values()? {
            (TokenType::SYMBOL, "[") => {
                self.consume(TokenType::SYMBOL, Some("["))?;
                self.consume(TokenType::NUMBER, None)?;
                self.consume(TokenType::SYMBOL, Some("]"))?;
            }
            _ => (),
        }

        Ok(())
    }

    fn asgn(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::OPERATOR, Some("="))
    }

    fn endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))
    }
}
