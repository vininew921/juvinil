use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

use super::scope::Scope;

pub struct Parser {
    tokens: Vec<Token>,
    pos: i32,
    current_token: Token,
    lookahead: Option<Token>,
    current_scope: Option<Scope>,
}

// General parsing methods (consuming, advancing tokens, etc)
impl Parser {
    pub fn new(tokens: Vec<Token>) -> JuvinilResult<Self> {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: Token::new(TokenType::EOF, String::new(), 0),
            lookahead: None,
            current_scope: Some(Scope::new(None)),
        };

        parser.next();

        Ok(parser)
    }

    pub fn parse(&mut self) -> JuvinilResult<()> {
        self.program()
    }

    fn push_scope(&mut self) {
        let parent = self.current_scope.take();
        self.current_scope = Some(Scope::new(parent));
    }

    fn pop_scope(&mut self) {
        let last_scope = self.current_scope.take();
        let parent = last_scope.unwrap().parent;
        self.current_scope = *parent;
    }

    fn search_scope(&mut self, var_name: String) -> bool {
        let mut scope = &self.current_scope;

        while let Some(inner_scope) = scope {
            if inner_scope
                .variables
                .iter()
                .any(|x| *x.var_name == var_name)
            {
                return true;
            }

            scope = &inner_scope.parent;
        }

        false
    }

    fn register_variable_in_scope(&mut self, var_type: String, var_name: String) {
        if let Some(current_scope) = self.current_scope.as_mut() {
            current_scope
                .variables
                .push(super::scope::JvVariable { var_type, var_name });
        }
    }

    fn assert_id_declared(&mut self) -> JuvinilResult<()> {
        //We collect the variable name
        //to check if it was already declared.
        //if it wasn't, we return an error
        let var_name = self.current_token.value.clone();
        if !self.search_scope(var_name.clone()) {
            return Err(JuvinilError::UndeclaredVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    fn assert_id_not_declared(&mut self) -> JuvinilResult<()> {
        //We collect the variable name
        //to check if it was already declared.
        //if it was, we return an error
        let var_name = self.current_token.value.clone();
        if self.search_scope(var_name.clone()) {
            return Err(JuvinilError::UndeclaredVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    fn next(&mut self) -> Option<&Token> {
        self.pos += 1;

        if self.pos >= self.tokens.len() as i32 {
            self.pos = (self.tokens.len() - 1) as i32;
        }

        let res = self.tokens.get(self.pos as usize);
        let lookahead = self.tokens.get(self.pos as usize + 1);

        if let Some(next) = res {
            tracing::info!("Next: {:?}", next);
        }

        self.current_token = res.unwrap().clone();
        self.lookahead = lookahead.cloned();

        res
    }

    fn consume(&mut self, token_type: TokenType, value: Option<&str>) -> JuvinilResult<()> {
        if self.current_token.token_type != token_type {
            return Err(JuvinilError::SyntaxError(
                token_type,
                value.unwrap_or_default().into(),
                self.current_token.clone(),
                self.current_token.file_line,
            ));
        }

        if value.is_some() && self.current_token.value != value.unwrap() {
            return Err(JuvinilError::SyntaxError(
                token_type,
                value.unwrap_or_default().into(),
                self.current_token.clone(),
                self.current_token.file_line,
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
        tracing::info!("PARSING PROGRAM");

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

        //If we're not at the end of the file, repeat!
        if self.current_token.token_type != TokenType::EOF {
            self.program()?;
        }

        Ok(())
    }

    //block -> { decls stmts }
    fn block(&mut self) -> JuvinilResult<()> {
        self.push_scope();

        tracing::info!("PARSING BLOCK");

        self.consume(TokenType::SYMBOL, Some("{"))?;

        //If the current token is an open bracket ({),
        //we're looking at a block
        while self.current_token.value != "}" {
            if self.current_token.value == "{" {
                self.block()?;
            }

            //If the current token type is a TYPE, we're
            //looking at a declaration (decls)
            if self.current_token.token_type == TokenType::TYPE {
                self.decls()?;
            }

            self.stmts()?;
        }

        self.consume(TokenType::SYMBOL, Some("}"))?;

        self.pop_scope();

        Ok(())
    }

    //decls -> decls decl
    fn decls(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING DECLS");

        //We run the declaration parsing until we no longer
        //have a TYPE as the current
        while self.current_token.token_type == TokenType::TYPE {
            self.decl()?;
        }

        Ok(())
    }

    //decl -> TYPE ID endexpr
    fn decl(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING DECL");

        let var_type = self.current_token.value.clone();
        self.jvtype()?;

        //Assert that the ID we're declaring wasn't
        //already declared
        self.assert_id_not_declared()?;

        let var_name = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;
        self.endexpr()?;

        self.register_variable_in_scope(var_type, var_name);

        Ok(())
    }

    //stmts -> stmts stmt
    fn stmts(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING STMTS");

        //Parse a statement if the current token is one
        //of the following values or types
        let stmt_values = [
            "for", "if", "while", "do", "break", "continue", "return", "{", "func",
        ];

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
        tracing::info!("PARSING STMT");

        //If current token is an ID, we're either looking at a function call (func)
        //or a assignment (asgn)
        if self.current_token.token_type == TokenType::ID {
            if let Some(lookahead) = self.lookahead.clone() {
                //It's a function call if the lookahead token is a parenthesis
                if lookahead.value == "(" {
                    self.func()?;
                    self.endexpr()?;

                    return Ok(());
                }

                //Otherwise, it's an assignment
                self.asgn()?;
            }

            return Ok(());
        }

        //Parse a block if the current token is a "{"
        if self.current_token.value == "{" {
            self.block()?;
            return Ok(());
        }

        //Parse a for expression if the current token is a for
        if self.current_token.value == "for" {
            self.stmt_for()?;
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

        //Parse a continue if the current token is 'continue'
        if self.current_token.value == "continue" {
            self.consume(TokenType::KEYWORD, Some("continue"))?;
            self.endexpr()?;
            return Ok(());
        }

        //If all of the above fail, the remaning condition
        //is to parse a 'return'
        self.stmt_return()?;
        Ok(())
    }

    fn stmt_return(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("return"))?;

        //return something (expr) if the current token is
        //not a semicolon
        if self.current_token.value != ";" {
            self.expr()?;
        }

        self.endexpr()?;
        Ok(())
    }

    //Parse a for expression
    fn stmt_for(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING STMT_FOR");

        self.consume(TokenType::KEYWORD, Some("for"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.asgn()?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        Ok(())
    }

    //Parse an if expression
    fn stmt_if(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING STMT_IF");

        self.consume(TokenType::KEYWORD, Some("if"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        if self.current_token.value == "else" {
            self.consume(TokenType::KEYWORD, Some("else"))?;
            self.block()?;
        }

        Ok(())
    }

    //Parse a while expression
    fn stmt_while(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING STMT_WHILE");

        self.consume(TokenType::KEYWORD, Some("while"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        Ok(())
    }

    //Parse a do while expression
    fn stmt_do_while(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING STMT_DO_WHILE");

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
        tracing::info!("PARSING BOOLEXPR");

        //A boolexpr will always end up as a join
        self.join()?;

        //After we consume a join, we check if we have the base case
        //for the boolexpr, which is a ||
        //if we do, we consume the comparator and rerun the boolexpr
        if self.current_token.value == "||" {
            self.consume(TokenType::COMPARATOR, None)?;
            self.boolexpr()?;
        }

        Ok(())
    }

    fn join(&mut self) -> JuvinilResult<()> {
        //A join will always end up as an equality
        self.equality()?;

        //After we consume an equality, we check if we have the base case
        //for the join, which is a &&
        //if we do, we consume the comparator and rerun the join
        if self.current_token.value == "&&" {
            self.consume(TokenType::COMPARATOR, None)?;
            self.join()?;
        }
        Ok(())
    }

    fn equality(&mut self) -> JuvinilResult<()> {
        //An equality will always end up as a cmp
        self.cmp()?;

        //After we consume a cmp, we check if we have the base case
        //for the equality, which is a == or !=
        //if we do, we consume the comparator and rerun the equality
        if self.current_token.value == "==" || self.current_token.value == "!=" {
            self.consume(TokenType::COMPARATOR, None)?;
            self.equality()?;
        }

        Ok(())
    }

    fn cmp(&mut self) -> JuvinilResult<()> {
        //The cmp is the easiest,
        //it's just an expression followed by a comparator followed by another expression
        self.expr()?;
        self.consume(TokenType::COMPARATOR, None)?;
        self.expr()?;

        Ok(())
    }

    //Parse an expression
    fn expr(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING EXPR");

        //An expr will always end up as a bnr
        self.bnr()?;

        //After we consume a bnr, we check if we have the base case
        //for the expr, which is a + or -
        //if we do, we consume the operator and rerun the expr
        if self.current_token.value == "+" || self.current_token.value == "-" {
            self.consume(TokenType::OPERATOR, None)?;
            self.expr()?;
        }

        Ok(())
    }

    fn bnr(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING BNR");

        //a bnr will always end up in a term
        self.term()?;

        //After we consume a term, we check if we have the base case
        //for the bnr, which is a & or |
        //if we do, we consume the operator an rerun the bnr
        if self.current_token.value == "&" || self.current_token.value == "|" {
            self.consume(TokenType::OPERATOR, None)?;
            self.bnr()?;
        }

        Ok(())
    }

    fn term(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING TERM");

        //a term will always end up in a unit
        self.unit()?;

        //After we consume a unit, we check if we have the base case
        //for the term, which is a *, / or %
        //if we do, we consume the operator an rerun the bnr
        if self.current_token.value == "*"
            || self.current_token.value == "/"
            || self.current_token.value == "%"
        {
            self.consume(TokenType::OPERATOR, None)?;
            self.term()?;
        }

        Ok(())
    }

    fn unit(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING UNIT");

        let unit_values = ["-", "++", "--"];
        if unit_values.contains(&self.current_token.value.as_str()) {
            self.consume(TokenType::OPERATOR, None)?;

            //Unfortunately, unit is recursive to the right,
            //so we can't use the same approach as the bnr
            self.unit()?;
        }

        self.factor()?;

        Ok(())
    }

    fn factor(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING FACTOR");

        if self.current_token.token_type == TokenType::NUMBER {
            self.consume(TokenType::NUMBER, None)?;
            return Ok(());
        }

        if self.current_token.value == "(" {
            self.consume(TokenType::SYMBOL, Some("("))?;
            self.expr()?;
            self.consume(TokenType::SYMBOL, Some(")"))?;
            return Ok(());
        }

        //If the current token is an ID and the lookahead isn't
        //a parenthesis, then it's just an ID asgn (x = y)
        if self.current_token.token_type == TokenType::ID {
            if let Some(lookahead) = self.lookahead.clone() {
                if lookahead.value != "(" {
                    self.assert_id_declared()?;

                    self.consume(TokenType::ID, None)?;
                    return Ok(());
                }
            }
        }

        //If the current token is not a number nor a parenthesis,
        //the only remaining option is for it to be a function
        self.func()?;

        Ok(())
    }

    //Parse a function declaration
    fn funcdecl(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING FUNCDECL");

        self.consume(TokenType::KEYWORD, Some("func"))?;
        self.jvtype()?;
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.paramsdecl()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        Ok(())
    }

    //Parse the parameters of a function declaration
    fn paramsdecl(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING PARAMSDECL");

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
        tracing::info!("PARSING FUNC");

        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.params()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        Ok(())
    }

    //Parse the parameters of a function call
    fn params(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING PARAMS");

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
            self.consume(TokenType::SYMBOL, Some(","))?;
            self.params()?;
        }

        Ok(())
    }

    //Parse an assignment
    fn asgn(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING ASGN");

        //Assert that the current variable
        //was declared before doing the assignment
        self.assert_id_declared()?;

        self.consume(TokenType::ID, None)?;

        //Match the current token value to check the operator
        match self.current_token.value.as_str() {
            "+=" => self.consume(TokenType::OPERATOR, Some("+="))?,
            "-=" => self.consume(TokenType::OPERATOR, Some("-="))?,
            _ => self.consume(TokenType::OPERATOR, Some("="))?,
        }

        //If the value after the operator is a string, consume it
        if self.current_token.token_type == TokenType::STRING {
            self.consume(TokenType::STRING, None)?;
            self.endexpr()?;
            return Ok(());
        }

        //Otherwise, consume an expression
        self.expr()?;
        self.endexpr()?;

        Ok(())
    }

    //Parse a TYPE expression
    //It can be a regular type or an ARRAY
    fn jvtype(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING JVTYPE");

        self.consume(TokenType::TYPE, None)?;

        if self.current_token.value == "[" {
            self.array_decl()?;
        }

        Ok(())
    }

    //Parse an array declaration
    //[ NUM ]
    fn array_decl(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING ARRAY_DECL");

        self.consume(TokenType::SYMBOL, Some("["))?;
        self.consume(TokenType::NUMBER, None)?;
        self.consume(TokenType::SYMBOL, Some("]"))?;

        Ok(())
    }

    //Parse the end of an expression, which is a ;
    fn endexpr(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING ENDEXPR");

        self.consume(TokenType::SYMBOL, Some(";"))?;

        Ok(())
    }
}
