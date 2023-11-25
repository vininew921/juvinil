use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Token,
}

// General parsing methods (consuming, advancing tokens, etc)
impl Parser {
    pub fn new(tokens: Vec<Token>) -> JuvinilResult<Self> {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: Token::new(TokenType::EOF, String::new()),
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
}

// Implementation of each of the source language parsing blocks
impl Parser {
    //Program is the first parse instruction of the whole file
    fn program(&mut self) -> JuvinilResult<()> {
        //If the current token is an open bracket ({),
        //we're looking at a block
        if self.current_token.value == "{" {
            self.block()?;
        }

        //If the current token type is a TYPE, we're
        //looking at a declaration (decls)
        if self.current_token.token_type == TokenType::TYPE {
            self.decls()?;
        }

        self.stmts()?;

        Ok(())
    }

    //block -> { decls stmts }
    fn block(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some("{"))?;
        self.decls()?;
        self.stmts()?;
        self.consume(TokenType::SYMBOL, Some("}"))?;

        Ok(())
    }

    //decls -> decls decl
    fn decls(&mut self) -> JuvinilResult<()> {
        //We run the declaration parsing until we no longer
        //have a TYPE as the current
        while self.current_token.token_type == TokenType::TYPE {
            self.decl()?;
        }

        Ok(())
    }

    //decl -> TYPE ID endexpr
    fn decl(&mut self) -> JuvinilResult<()> {
        self.jvtype()?;
        self.consume(TokenType::ID, None)?;
        self.endexpr()?;

        Ok(())
    }

    //stmts -> stmts stmt
    fn stmts(&mut self) -> JuvinilResult<()> {
        //Parse a statement if the current token is one
        //of the following values or types
        let stmt_values = ["if", "while", "do", "break", "continue", "{", "func"];
        let stmt_types = [TokenType::ID];

        while stmt_values.contains(&self.current_token.value.as_str())
            || stmt_types.contains(&self.current_token.token_type)
        {
            self.stmt()?;
        }

        Ok(())
    }

    //Statement can be pretty much everything that is not a declaration
    fn stmt(&mut self) -> JuvinilResult<()> {
        //If current token is an ID, we're either looking at a function call (func)
        //or a assignment (asgn)
        if self.current_token.token_type == TokenType::ID {
            self.consume(TokenType::ID, None)?;

            //It's a function call if the current token is a parenthesis
            if self.current_token.value == "(" {
                self.func()?;

                return Ok(());
            }

            //Otherwise, it's an assignment
            self.asgn()?;

            return Ok(());
        }

        //Parse a block if the current token is a "{"
        if self.current_token.value == "{" {
            self.block()?;
            return Ok(());
        }

        //Parse an if expression if the current token is an if
        if self.current_token.value == "if" {
            self.stmt_if()?;
            return Ok(());
        }

        //Parse a while expression if the current token is a while
        if self.current_token.value == "while" {
            self.stmt_while()?;
            return Ok(());
        }

        //Parse a do while expression if the current token is a do while
        if self.current_token.value == "do" {
            self.stmt_do_while()?;
            return Ok(());
        }

        //Parse a function declaration if the current token is a func
        if self.current_token.value == "func" {
            self.funcdecl()?;
            return Ok(());
        }

        //Parse a break if the current token is 'break'
        if self.current_token.value == "break" {
            self.consume(TokenType::KEYWORD, Some("break"))?;
            self.endexpr()?;
            return Ok(());
        }

        //If all of the above fail, the remaning condition
        //is to parse a 'continue'
        self.consume(TokenType::KEYWORD, Some("continue"))?;
        self.endexpr()?;
        Ok(())
    }

    //Parse an if expression
    fn stmt_if(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("if"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        if self.current_token.value == "else" {
            self.consume(TokenType::KEYWORD, Some("else"))?;
            self.block()?;
        }

        Ok(())
    }

    //Parse a while expression
    fn stmt_while(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("while"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        Ok(())
    }

    //Parse a do while expression
    fn stmt_do_while(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("do"))?;
        self.block()?;
        self.consume(TokenType::KEYWORD, Some("while"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.endexpr()?;

        Ok(())
    }

    //Parse a boolean expression
    fn boolexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::ID, None) //TODO
    }

    //Parse a function declaration
    fn funcdecl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("func"))?;
        self.consume(TokenType::TYPE, None)?;
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.paramsdecl()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        Ok(())
    }

    //Parse the parameters of a function declaration
    fn paramsdecl(&mut self) -> JuvinilResult<()> {
        //If the current token is a closing brackets ')',
        //then there are no parameters and we return early
        if self.current_token.value == ")" {
            return Ok(());
        }

        //Consume a parameter declaration
        self.consume(TokenType::TYPE, None)?;
        self.consume(TokenType::ID, None)?;

        //If the current token is a comma (,)
        //We consume a new paramsdecl
        if self.current_token.value == "," {
            self.paramsdecl()?;
        }

        Ok(())
    }

    //Parse a function call
    fn func(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.params()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        Ok(())
    }

    //Parse the parameters of a function call
    fn params(&mut self) -> JuvinilResult<()> {
        //If the current token is a closing brackets ')',
        //then there are no parameters and we return early
        if self.current_token.value == ")" {
            return Ok(());
        }

        //Consume an ID
        self.consume(TokenType::ID, None)?;

        //If the current token is a comma (,)
        //We consume a new param
        if self.current_token.value == "," {
            self.params()?;
        }

        Ok(())
    }

    //Parse a TYPE expression
    //It can be a regular type or an ARRAY
    fn jvtype(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;

        if self.current_token.value == "[" {
            self.array_decl()?;
        }

        Ok(())
    }

    //Parse an array declaration
    //[ NUM ]
    fn array_decl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some("["))?;
        self.consume(TokenType::NUMBER, None)?;
        self.consume(TokenType::SYMBOL, Some("]"))?;

        Ok(())
    }

    //Parse an assignment
    //TODO: Consume a boolexpr or expr after the =
    fn asgn(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::OPERATOR, Some("="))?;

        Ok(())
    }

    //Parse the end of an expression, which is a ;
    fn endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))?;

        Ok(())
    }
}
