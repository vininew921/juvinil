use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

use super::parser_map::ParserMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Option<Token>,
    _map: ParserMap,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> JuvinilResult<Self> {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: None,
            _map: ParserMap::new()?,
        };

        parser.next();

        Ok(parser)
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

        if value.is_some() && ref_self.value != value.unwrap() {
            return Err(JuvinilError::SyntaxError);
        }

        self.next();

        Ok(())
    }

    fn get_current_values(&self) -> JuvinilResult<(TokenType, &str)> {
        if self.current_token.is_none() {
            return Err(JuvinilError::ParsingError);
        }

        let values = self.current_token.as_ref().unwrap().values();

        Ok(values)
    }

    //program -> block
    //program -> decls
    //program -> stmts
    fn program(&mut self) -> JuvinilResult<()> {
        match self.get_current_values()? {
            (TokenType::SYMBOL, "{") => self.block(),
            (TokenType::TYPE, ..) => self.decls(),
            _ => self.stmts(),
        }
    }

    //block -> { decls stmts }
    fn block(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some("{"))?;
        self.decls()?;
        self.stmts()?;
        self.consume(TokenType::SYMBOL, Some("}"))
    }

    //decls -> decls decl
    fn decls(&mut self) -> JuvinilResult<()> {
        while self.get_current_values()?.0 == TokenType::TYPE {
            self.decl()?;
        }

        Ok(())
    }

    //decl -> type ID endexpr
    fn decl(&mut self) -> JuvinilResult<()> {
        self.jvtype()?;
        self.consume(TokenType::ID, None)?;
        self.endexpr()
    }

    //stmts -> stmts stmt
    fn stmts(&mut self) -> JuvinilResult<()> {
        self.stmt()
    }

    //TO DO: mapping
    fn stmt(&mut self) -> JuvinilResult<()> {
        if let (TokenType::ID, ..) = self.get_current_values()? {
            self.asgn()?
        }

        Ok(())
    }

    //type -> TYPE
    //type -> TYPE array_decl
    fn jvtype(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;

        if let (TokenType::SYMBOL, "[") = self.get_current_values()? {
            self.array_decl()?
        }

        Ok(())
    }

    //array_decl -> [ NUMBER ]
    fn array_decl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some("["))?;
        self.consume(TokenType::NUMBER, None)?;
        self.consume(TokenType::SYMBOL, Some("]"))
    }

    //asgn -> ID =
    fn asgn(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::OPERATOR, Some("="))
    }

    //endexpr -> ;
    fn endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))
    }
}
