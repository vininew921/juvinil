use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

use super::parser_map::ParserMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Token,
    map: ParserMap,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> JuvinilResult<Self> {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: Token::new(TokenType::EOF, String::new()),
            map: ParserMap::new()?,
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

        self.current_token = res.unwrap().clone();

        res
    }

    fn consume(&mut self, token_type: TokenType, value: Option<&str>) -> JuvinilResult<()> {
        if self.current_token.token_type != token_type {
            return Err(JuvinilError::SyntaxError(
                token_type,
                value.unwrap_or_default().into(),
                self.current_token.clone(),
            ));
        }

        if value.is_some() && self.current_token.value != value.unwrap() {
            return Err(JuvinilError::SyntaxError(
                token_type,
                value.unwrap_or_default().into(),
                self.current_token.clone(),
            ));
        }

        self.next();

        Ok(())
    }

    //program -> block
    //program -> decls
    //program -> stmts
    fn program(&mut self) -> JuvinilResult<()> {
        if self.map.is_first("block", &self.current_token) {
            return self.block();
        }

        if self.map.is_first("decls", &self.current_token) {
            return self.decls();
        }

        self.stmts()
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
        while self.map.is_first("decl", &self.current_token) {
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
        if self.map.is_first("asgn", &self.current_token) {
            self.asgn()?
        }

        Ok(())
    }

    //type -> TYPE
    //type -> TYPE array_decl
    fn jvtype(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;

        if self.map.is_follow("type", &self.current_token) {
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
