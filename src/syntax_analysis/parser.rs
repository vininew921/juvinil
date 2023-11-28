use std::fs::{self};

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
    scope_level: usize,           //Current scope level to determine tabs in intermediary code
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
            intermediary_code: String::from(
                "#include <stdio.h>\n#include <string>\n#include <cstdlib>\n#include <conio.h>\nusing namespace std;\n\n",
            ),
            scope_level: 0
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
        self.start()?;
        Ok(())
    }

    fn push_intermediary_code(&mut self, text: &str) {
        self.intermediary_code.push_str(text);
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
        self.scope_level += 1;
    }

    //Take the current scope and throw it away,
    //making the scope's parent the new current scope
    fn pop_scope(&mut self) {
        let last_scope = self.current_scope.take();
        let parent = last_scope.unwrap().parent;
        self.current_scope = *parent;
        self.scope_level -= 1;
    }

    //Search for a variable inside the current scope
    //If the variable wasn't found, we search recursively
    //through the scope's parent until we find it or the
    //parent is null
    fn search_var_in_scope(&mut self, var_name: String) -> Option<JvVariable> {
        let mut scope = &self.current_scope;

        while let Some(inner_scope) = scope {
            let search_result = inner_scope
                .variables
                .iter()
                .find(|x| x.var_name == var_name);

            if let Some(result) = search_result {
                return Some(result.clone());
            }

            scope = &inner_scope.parent;
        }

        None
    }

    //Search for a function inside the current scope
    //If the function wasn't found, we search recursively
    //through the scope's parent until we find it or the
    //parent is null
    fn search_func_in_scope(&mut self, func_name: String) -> Option<JvFunction> {
        let mut scope = &self.current_scope;

        while let Some(inner_scope) = scope {
            let search_result = inner_scope
                .functions
                .iter()
                .find(|x| x.func_name == func_name);

            if let Some(result) = search_result {
                return Some(result.clone());
            }

            scope = &inner_scope.parent;
        }

        None
    }

    //Register a variable in the current scope
    fn register_variable_in_scope(&mut self, var_type: String, var_name: String) {
        tracing::info!(
            "Registering variable `{}` in scope {}",
            var_name,
            self.scope_level
        );

        if let Some(current_scope) = self.current_scope.as_mut() {
            current_scope.variables.push(JvVariable {
                var_type,
                var_name,
                assigned: false,
            });
        }
    }

    fn mark_variable_as_assigned(&mut self, var_name: String) -> JuvinilResult<()> {
        let mut scope = &mut self.current_scope;

        while let Some(inner_scope) = scope {
            let search_result = inner_scope
                .variables
                .iter_mut()
                .find(|x| x.var_name == var_name);

            if let Some(result) = search_result {
                result.assigned = true;
                return Ok(());
            }

            scope = &mut inner_scope.parent;
        }

        Err(JuvinilError::UndeclaredVariable(
            var_name,
            self.current_token.file_line,
        ))
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it wasn't, we return an error
    fn assert_id_declared(&mut self, declared: bool) -> JuvinilResult<Option<JvVariable>> {
        let var_name = self.current_token.value.clone();
        let variable = self.search_var_in_scope(var_name.clone());

        if variable.is_none() && declared {
            return Err(JuvinilError::UndeclaredVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        if variable.is_some() && !declared {
            return Err(JuvinilError::DuplicateVariable(
                var_name,
                self.current_token.file_line,
            ));
        }

        Ok(variable)
    }

    fn assert_id_assigned(&mut self) -> JuvinilResult<()> {
        let var_name = self.current_token.value.clone();
        let variable = self.search_var_in_scope(var_name.clone());

        if let Some(result) = variable {
            if result.assigned {
                return Ok(());
            }
        }

        return Err(JuvinilError::UnassignedVariable(
            var_name,
            self.current_token.file_line,
        ));
    }

    //Register a function in the current scope
    fn register_func_in_scope(
        &mut self,
        return_type: String,
        func_name: String,
        params: Vec<String>,
    ) {
        if let Some(current_scope) = self.current_scope.as_mut() {
            current_scope.functions.push(JvFunction {
                return_type,
                func_name,
                params,
            });
        }
    }

    //We collect the variable name
    //to check if it was already declared.
    //if it wasn't, we return an error
    fn assert_func_declared(&mut self, declared: bool) -> JuvinilResult<Option<JvFunction>> {
        let func_name = self.current_token.value.clone();
        let function = self.search_func_in_scope(func_name.clone());

        if function.is_some() != declared {
            return Err(JuvinilError::UndeclaredFunction(
                func_name,
                self.current_token.file_line,
            ));
        }

        Ok(function)
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
    //Start is the first parse instruction
    //First it parses all function declarations,
    //then parses the rest of the program
    fn start(&mut self) -> JuvinilResult<()> {
        //Parse all function declarations
        //before the main program
        while self.current_token.value == "func" {
            self.funcdecl()?;
        }

        self.push_intermediary_code("int main() {\n");

        self.program()?;

        self.push_intermediary_code("\n\ngetch();\n");
        self.push_intermediary_code("}\n");

        Ok(())
    }

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

        //If we're not at the end of the file, repeat!
        if self.current_token.token_type != TokenType::EOF {
            self.program()?;
        }

        Ok(())
    }

    //block -> { decls stmts }
    fn block(&mut self) -> JuvinilResult<()> {
        self.push_scope();

        self.consume(TokenType::SYMBOL, Some("{"))?;
        self.push_intermediary_code("{\n");

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
        self.push_intermediary_code("}\n");

        self.pop_scope();

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
        let var_type = self.current_token.value.clone();
        self.jvtype()?;

        //Assert that the ID we're declaring wasn't
        //already declared
        self.assert_id_declared(false)?;

        let var_name = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;
        self.endexpr()?;

        self.intermediary_code
            .push_str(format!("{} {};\n", self.map_type(var_type.as_str()), var_name).as_str());

        self.register_variable_in_scope(var_type, var_name);

        Ok(())
    }

    //stmts -> stmts stmt
    fn stmts(&mut self) -> JuvinilResult<()> {
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
        //If current token is an ID, we're either looking at a function call (func)
        //or a assignment (asgn)
        if self.current_token.token_type == TokenType::ID {
            if let Some(lookahead) = self.lookahead.clone() {
                //It's a function call if the lookahead token is a parenthesis
                if lookahead.value == "(" {
                    let func_value = self.func()?;
                    self.endexpr()?;

                    self.intermediary_code
                        .push_str(format!("{};\n", func_value).as_str());

                    return Ok(());
                }

                //Otherwise, it's an assignment
                self.asgn()?;
                self.push_intermediary_code("\n");
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

        //Parse a break if the current token is 'break'
        if self.current_token.value == "break" {
            self.consume(TokenType::KEYWORD, Some("break"))?;
            self.endexpr()?;

            self.push_intermediary_code("break;\n");
            return Ok(());
        }

        //Parse a continue if the current token is 'continue'
        if self.current_token.value == "continue" {
            self.consume(TokenType::KEYWORD, Some("continue"))?;
            self.endexpr()?;
            self.push_intermediary_code("continue;\n");
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
        let mut expr_result = String::new();
        if self.current_token.value != ";" {
            if self.current_token.token_type == TokenType::STRING {
                expr_result = format!("\"{}\"", self.current_token.value);
                self.consume(TokenType::STRING, None)?;
            } else {
                expr_result = self.expr()?;
            }
        }

        self.endexpr()?;

        self.intermediary_code
            .push_str(format!("return {};\n", expr_result).as_str());

        Ok(())
    }

    //Parse a for expression
    fn stmt_for(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("for"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;

        self.push_intermediary_code("for(");

        self.asgn()?;
        let boolexpr_result = self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        self.intermediary_code
            .push_str(format!(" {} ;)", boolexpr_result).as_str());

        self.block()?;

        Ok(())
    }

    //Parse an if expression
    fn stmt_if(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("if"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        let boolexpr_result = self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        let if_expr = format!("if ({})", boolexpr_result);
        self.push_intermediary_code(if_expr.as_str());

        self.block()?;

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
        let boolexpr_result = self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        self.intermediary_code
            .push_str(format!("while({}) ", boolexpr_result).as_str());

        self.block()?;

        Ok(())
    }

    //Parse a do while expression
    fn stmt_do_while(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("do"))?;
        self.push_intermediary_code("do ");

        self.block()?;
        self.consume(TokenType::KEYWORD, Some("while"))?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        let boolexpr_result = self.boolexpr()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;
        self.endexpr()?;

        self.intermediary_code
            .push_str(format!("while({});\n", boolexpr_result).as_str());

        Ok(())
    }

    //Parse a boolean expression
    fn boolexpr(&mut self) -> JuvinilResult<String> {
        let mut result = String::new();

        //A boolexpr will always end up as a join
        let join_result = self.join()?;
        result.push_str(join_result.as_str());

        //After we consume a join, we check if we have the base case
        //for the boolexpr, which is a ||
        //if we do, we consume the comparator and rerun the boolexpr
        if self.current_token.value == "||" {
            let comparator = self.current_token.value.clone();

            self.consume(TokenType::COMPARATOR, None)?;
            let recursive_result = self.boolexpr()?;

            result.push_str(format!(" {} {}", comparator, recursive_result).as_str());
        }

        Ok(result)
    }

    fn join(&mut self) -> JuvinilResult<String> {
        let mut result = String::new();

        //A join will always end up as an equality
        let equality_result = self.equality()?;
        result.push_str(equality_result.as_str());

        //After we consume an equality, we check if we have the base case
        //for the join, which is a &&
        //if we do, we consume the comparator and rerun the join
        if self.current_token.value == "&&" {
            let comparator = self.current_token.value.clone();
            self.consume(TokenType::COMPARATOR, None)?;
            let recursive_result = self.join()?;

            result.push_str(format!(" {} {}", comparator, recursive_result).as_str());
        }

        Ok(result)
    }

    fn equality(&mut self) -> JuvinilResult<String> {
        let mut result = String::new();
        //An equality will always end up as a cmp
        let cmp_result = self.cmp()?;
        result.push_str(cmp_result.as_str());

        //After we consume a cmp, we check if we have the base case
        //for the equality, which is a == or !=
        //if we do, we consume the comparator and rerun the equality
        if self.current_token.value == "==" || self.current_token.value == "!=" {
            let comparator = self.current_token.value.clone();
            self.consume(TokenType::COMPARATOR, None)?;
            let recursive_result = self.equality()?;

            result.push_str(recursive_result.as_str());
            result.push_str(format!(" {} {}", comparator, recursive_result).as_str());
        }

        Ok(result)
    }

    fn cmp(&mut self) -> JuvinilResult<String> {
        //The cmp is the easiest,
        //it's just an expression followed by a comparator followed by another expression
        let expr1_result = self.expr()?;
        let comparator = self.current_token.value.clone();
        self.consume(TokenType::COMPARATOR, None)?;
        let expr2_result = self.expr()?;

        let result = format!("{} {} {}", expr1_result, comparator, expr2_result);

        Ok(result.into())
    }

    //Parse an expression
    fn expr(&mut self) -> JuvinilResult<String> {
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
                    //Assert that the ID is assigned before being used
                    self.assert_id_assigned()?;

                    let var_name = self.current_token.value.clone();
                    self.consume(TokenType::ID, None)?;

                    return Ok(var_name);
                }
            }
        }

        //If the token is a primitive true or false,
        //consume the respective keywords
        if self.current_token.value == "true" || self.current_token.value == "false" {
            let bool_value = self.current_token.value.clone();
            self.consume(TokenType::KEYWORD, None)?;

            return Ok(bool_value);
        }

        //If the current token is not a number nor a parenthesis,
        //the only remaining option is for it to be a function
        let func_result = self.func()?;

        Ok(func_result)
    }

    //Parse a function declaration
    fn funcdecl(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::KEYWORD, Some("func"))?;

        let return_type = self.map_type(self.current_token.value.clone().as_str());
        self.jvtype()?;

        //Assert that the function hasn't already
        //been declared
        self.assert_func_declared(false)?;

        let func_name = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;

        //Enter a new scope to register
        //function variables
        self.push_scope();

        self.consume(TokenType::SYMBOL, Some("("))?;
        let params_value = self.paramsdecl()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        self.intermediary_code
            .push_str(format!("{} {} ({})", return_type, func_name, params_value).as_str());

        self.block()?;

        //Leave function scope after block ends
        self.pop_scope();

        //Extract params from `params_value` string
        let func_params: Vec<String> = params_value
            .split(',')
            .map(|s| {
                s.trim()
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
            .collect();

        self.register_func_in_scope(return_type, func_name, func_params);

        Ok(())
    }

    //Parse the parameters of a function declaration
    fn paramsdecl(&mut self) -> JuvinilResult<String> {
        let mut params_decl_result = String::new();

        //If the current token is a closing brackets ')',
        //then there are no parameters and we return early
        if self.current_token.value == ")" {
            return Ok(params_decl_result);
        }

        //Consume a parameter declaration
        let param_type_value = self.current_token.value.clone();
        self.consume(TokenType::TYPE, None)?;

        let id_value = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;

        //Register variable in current scope
        //and also mark it as declared, since
        //it's a value that's coming from outside
        self.register_variable_in_scope(param_type_value.clone(), id_value.clone());
        self.mark_variable_as_assigned(id_value.clone())?;

        params_decl_result.push_str(
            format!("{} {}", self.map_type(param_type_value.as_str()), id_value).as_str(),
        );

        //If the current token is a comma (,)
        //We consume a new paramsdecl
        if self.current_token.value == "," {
            self.consume(TokenType::SYMBOL, Some(","))?;
            params_decl_result.push_str(",");
            params_decl_result.push_str(self.paramsdecl()?.as_str());
        }

        Ok(params_decl_result)
    }

    //Parse a function call
    fn func(&mut self) -> JuvinilResult<String> {
        //Assert that the current function
        //has already been declared, unless it's the printf function
        let func_name = self.current_token.value.clone();
        if func_name != "printf" {
            self.assert_func_declared(true)?;
        }

        self.consume(TokenType::ID, None)?;
        self.consume(TokenType::SYMBOL, Some("("))?;
        let func_params = self.params()?;
        self.consume(TokenType::SYMBOL, Some(")"))?;

        Ok(format!("{} ({})", func_name, func_params))
    }

    //Parse the parameters of a function call
    fn params(&mut self) -> JuvinilResult<String> {
        let mut params_result = String::new();

        //If the current token is a closing brackets ')',
        //then there are no parameters and we return early
        if self.current_token.value == ")" {
            return Ok(String::new());
        }

        let param_result: String;

        //If the parameter is not a STRING, then
        //it is any expression
        if self.current_token.token_type == TokenType::STRING {
            param_result = format!("\"{}\"", self.current_token.value.clone());
            self.consume(TokenType::STRING, None)?;
        } else {
            //Parameters can be any expression
            param_result = self.expr()?;
        }

        params_result.push_str(param_result.as_str());

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
        //Assert that the current variable
        //was declared before doing the assignment
        self.assert_id_declared(true)?;

        let id_value = self.current_token.value.clone();
        self.consume(TokenType::ID, None)?;

        self.mark_variable_as_assigned(id_value.clone())?;

        //Match the current token value to check the operator
        let operator_value = self.current_token.value.clone();
        match self.current_token.value.as_str() {
            "+=" => self.consume(TokenType::OPERATOR, Some("+="))?,
            "-=" => self.consume(TokenType::OPERATOR, Some("-="))?,
            _ => self.consume(TokenType::OPERATOR, Some("="))?,
        }

        //If the value after the operator is a string, consume it
        let expr_value: String;
        if self.current_token.token_type == TokenType::STRING {
            expr_value = format!("\"{}\"", self.current_token.value.clone());
            self.consume(TokenType::STRING, None)?;
        } else {
            //Otherwise, consume an expression
            expr_value = self.expr()?;
        }

        self.endexpr()?;

        self.intermediary_code
            .push_str(format!("{} {} {};", id_value, operator_value, expr_value).as_str());

        Ok(())
    }

    //Parse a TYPE expression
    //It can be a regular type or an ARRAY
    fn jvtype(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::TYPE, None)?;

        Ok(())
    }

    //Parse the end of an expression, which is a ;
    fn endexpr(&mut self) -> JuvinilResult<()> {
        self.consume(TokenType::SYMBOL, Some(";"))?;

        Ok(())
    }
}
