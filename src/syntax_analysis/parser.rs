use std::fs;

use crate::{
    error::{JuvinilError, JuvinilResult},
    lexical_analysis::token::{Token, TokenType},
};

use super::scope::{JvFunction, JvVariable, Scope};

pub struct Parser {
    tokens: Vec<Token>,           //List of tokens created by the lexical analyzer
    pos: i32,                     //Current position in the token list
    current_token: Token,         //Reference to the current token (tokens[pos])
    lookahead: Option<Token>,     //Reference to the lookahead (tokens[pos + 1])
    current_scope: Option<Scope>, //Current active scope
    intermediary_code: String,    //Intermediary code generated during the parse routine
}

// General parsing methods (consuming, advancing tokens, etc)
impl Parser {
    //Instantiates a new `Parser` object, initializing all variables
    //with their default values
    pub fn new(tokens: Vec<Token>) -> JuvinilResult<Self> {
        let mut parser = Parser {
            tokens,
            pos: -1,
            current_token: Token::new(TokenType::EOF, String::new(), 0),
            lookahead: None,
            current_scope: Some(Scope::new(None)),
            intermediary_code: String::from("#include <stdio.h>\n\n"),
        };

        //We call the `next()` function
        //to set the current token to be the first token
        //of the lexical analisys output.
        parser.next();

        Ok(parser)
    }

    //Dumps the intermediary code generated during the
    //parse routine to the specified file
    pub fn dump_intermediary_code(&self, filepath: &str) -> JuvinilResult<()> {
        fs::write(filepath, self.intermediary_code.clone())?;
        Ok(())
    }

    //Initiates the parsing routine, starting with the `program`
    pub fn parse(&mut self) -> JuvinilResult<()> {
        self.intermediary_code.push_str("int main() {\n");
        self.program()?;
        self.intermediary_code.push_str("}\n");

        Ok(())
    }

    //Maps a JvType to a C type
    fn map_type(&self, value: &str) -> String {
        match value {
            "void" => "void".into(),
            "int" => "int".into(),
            "float" => "float".into(),
            "boolean" => "bool".into(),
            "char" => "char".into(),
            "string" => "string".into(),
            _ => panic!("Wtf"),
        }
    }

    //Create a new scope and set the current scope as the
    //parent of the new scope. Then, make the
    //new scope the current scope
    fn push_scope(&mut self) {
        let parent = self.current_scope.take();
        self.current_scope = Some(Scope::new(parent));
    }

    //Take the current scope and throw it away,
    //making the scope's parent the new current scope
    fn pop_scope(&mut self) {
        let last_scope = self.current_scope.take();
        let parent = last_scope.unwrap().parent;
        self.current_scope = *parent;
    }

    //Search for a variable inside the current scope
    //If the variable wasn't found, we search recursively
    //through the scope's parent until we find it or the
    //parent is null
    fn search_var_in_scope(&mut self, var_name: String) -> bool {
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

    //Search for a function inside the current scope
    //If the function wasn't found, we search recursively
    //through the scope's parent until we find it or the
    //parent is null
    fn search_func_in_scope(&mut self, func_name: String) -> bool {
        let mut scope = &self.current_scope;

        while let Some(inner_scope) = scope {
            if inner_scope
                .functions
                .iter()
                .any(|x| *x.func_name == func_name)
            {
                return true;
            }

            scope = &inner_scope.parent;
        }

        false
    }

    //Register a variable in the current scope
    fn register_variable_in_scope(&mut self, var_type: String, var_name: String) {
        if let Some(current_scope) = self.current_scope.as_mut() {
            current_scope
                .variables
                .push(JvVariable { var_type, var_name });
        }
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it wasn't, we return an error
    fn assert_id_declared(&mut self) -> JuvinilResult<()> {
        let var_name = self.current_token.value.clone();
        if !self.search_var_in_scope(var_name.clone()) {
            return Err(JuvinilError::UndeclaredVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it was, we return an error
    fn assert_id_not_declared(&mut self) -> JuvinilResult<()> {
        let var_name = self.current_token.value.clone();
        if self.search_var_in_scope(var_name.clone()) {
            return Err(JuvinilError::DuplicateVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    //Register a function in the current scope
    fn register_func_in_scope(&mut self, return_type: String, func_name: String) {
        if let Some(current_scope) = self.current_scope.as_mut() {
            current_scope.functions.push(JvFunction {
                return_type,
                func_name,
            });
        }
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it wasn't, we return an error
    fn assert_func_declared(&mut self) -> JuvinilResult<()> {
        let func_name = self.current_token.value.clone();
        if !self.search_func_in_scope(func_name.clone()) {
            return Err(JuvinilError::UndeclaredFunction(
                func_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it was, we return an error
    fn assert_func_not_declared(&mut self) -> JuvinilResult<()> {
        let func_name = self.current_token.value.clone();
        if self.search_func_in_scope(func_name.clone()) {
            return Err(JuvinilError::DuplicateFunction(
                func_name,
                self.current_token.file_line,
            ));
        }

        Ok(())
    }

    //Iterate over the list of tokens.
    //We make the token at position `self.pos` the current token,
    //and the token at `self.pos + 1` the lookahead token
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

    //Consume a token. If the token is of a different type or value
    //than the provided values, we throw a Syntax Error.
    //We only check for the token `value` property if an actual value
    //was provided. This means that we ignore the `value` check if
    //a None value is given
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
    //Intermediary code OK
    fn block(&mut self) -> JuvinilResult<()> {
        self.push_scope();

        tracing::info!("PARSING BLOCK");

        self.consume(TokenType::SYMBOL, Some("{"))?;
        self.intermediary_code.push_str("{\n");

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
        self.intermediary_code.push_str("}\n\n");

        self.pop_scope();

        Ok(())
    }

    //decls -> decls decl
    //Intermediary code OK
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
    //Intermediary code OK
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

        self.intermediary_code
            .push_str(format!("{} {};\n", self.map_type(var_type.as_str()), var_name).as_str());

        self.register_variable_in_scope(var_type, var_name);

        Ok(())
    }

    //stmts -> stmts stmt
    //Intermediary code OK
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
    //Intermediary code TODO
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
    fn expr(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING EXPR");

        let mut expr_result = String::new();

        //An expr will always end up as a bnr
        let bnr_result = self.bnr()?;
        expr_result.push_str(bnr_result.as_str());

        //After we consume a bnr, we check if we have the base case
        //for the expr, which is a + or -
        //if we do, we consume the operator and rerun the expr
        if self.current_token.value == "+" || self.current_token.value == "-" {
            let operator = self.current_token.value.clone();
            self.consume(TokenType::OPERATOR, None)?;
            expr_result.push_str(operator.as_str());

            let recursive_result = self.expr()?;
            expr_result.push_str(recursive_result.as_str());
        }

        Ok(expr_result)
    }

    fn bnr(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING BNR");

        let mut bnr_result = String::new();

        //a bnr will always end up in a term
        let term_result = self.term()?;
        bnr_result.push_str(term_result.as_str());

        //After we consume a term, we check if we have the base case
        //for the bnr, which is a & or |
        //if we do, we consume the operator an rerun the bnr
        if self.current_token.value == "&" || self.current_token.value == "|" {
            let operator = self.current_token.value.clone();
            self.consume(TokenType::OPERATOR, None)?;
            bnr_result.push_str(operator.as_str());

            let recursive_result = self.bnr()?;
            bnr_result.push_str(recursive_result.as_str());
        }

        Ok(bnr_result)
    }

    fn term(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING TERM");

        let mut term_result = String::new();

        //a term will always end up in a unit
        let unit_result = self.unit()?;
        term_result.push_str(unit_result.as_str());

        //After we consume a unit, we check if we have the base case
        //for the term, which is a *, / or %
        //if we do, we consume the operator an rerun the bnr
        if self.current_token.value == "*"
            || self.current_token.value == "/"
            || self.current_token.value == "%"
        {
            let operator = self.current_token.value.clone();
            self.consume(TokenType::OPERATOR, None)?;
            term_result.push_str(operator.as_str());

            let recursive_result = self.term()?;
            term_result.push_str(recursive_result.as_str());
        }

        Ok(term_result)
    }

    //A unit is a raw number that can have a modifier (++, --, -)
    fn unit(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING UNIT");

        let mut unit_result = String::new();

        let unit_values = ["-", "++", "--"];
        if unit_values.contains(&self.current_token.value.as_str()) {
            let operator = self.current_token.value.clone();
            self.consume(TokenType::OPERATOR, None)?;
            unit_result.push_str(operator.as_str());

            //Unfortunately, unit is recursive to the right,
            //so we can't use the same approach as the bnr
            let recursive_result = self.unit()?;
            unit_result.push_str(recursive_result.as_str());
        }

        let factor_result = self.factor()?;
        unit_result.push_str(factor_result.as_str());

        Ok(unit_result)
    }

    //A factor is basically a raw number or variable
    fn factor(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING FACTOR");

        if self.current_token.token_type == TokenType::NUMBER {
            let number = self.current_token.value.clone();
            self.consume(TokenType::NUMBER, None)?;

            return Ok(format!("{}", number));
        }

        if self.current_token.value == "(" {
            self.consume(TokenType::SYMBOL, Some("("))?;
            let expr_result = self.expr()?;
            self.consume(TokenType::SYMBOL, Some(")"))?;

            return Ok(format!("({})", expr_result));
        }

        //If the current token is an ID and the lookahead isn't
        //a parenthesis, then it's just an ID asgn (x = y)
        if self.current_token.token_type == TokenType::ID {
            if let Some(lookahead) = self.lookahead.clone() {
                if lookahead.value != "(" {
                    self.assert_id_declared()?;

                    let var_name = self.current_token.value.clone();
                    self.consume(TokenType::ID, None)?;

                    return Ok(var_name);
                }
            }
        }

        //If the current token is not a number nor a parenthesis,
        //the only remaining option is for it to be a function
        let func_result = self.func()?;

        Ok(func_result)
    }

    //Parse a function declaration
    fn funcdecl(&mut self) -> JuvinilResult<()> {
        tracing::info!("PARSING FUNCDECL");

        self.consume(TokenType::KEYWORD, Some("func"))?;

        let return_type = self.current_token.value.clone();
        self.jvtype()?;

        //Assert that the function hasn't already
        //been declared
        self.assert_func_not_declared()?;
        let func_name = self.current_token.value.clone();

        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        self.paramsdecl()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.block()?;

        self.register_func_in_scope(return_type, func_name);

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
    //Intermediary code OK
    fn func(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING FUNC");

        //Assert that the current function
        //has already been declared
        self.assert_func_declared()?;

        let func_name = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        let func_params = self.params()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        Ok(format!("{} ( {} );", func_name, func_params))
    }

    //Parse the parameters of a function call
    fn params(&mut self) -> JuvinilResult<String> {
        tracing::info!("PARSING PARAMS");

        let mut params_result = String::new();

        //If the current token is a closing brackets ')',
        //then there are no parameters and we return early
        if self.current_token.value == ")" {
            return Ok(String::new());
        }

        //Parameters can be any expression
        let expr_result = self.expr()?;
        params_result.push_str(expr_result.as_str());

        //If the current token is a comma (,)
        //We consume a new param
        if self.current_token.value == "," {
            self.consume(TokenType::SYMBOL, Some(","))?;
            params_result.push_str(", ");
            let recursive_result = self.params()?;
            params_result.push_str(recursive_result.as_str());
        }

        Ok(params_result)
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
