use language::*;
use lexer::*;

use std::os::raw::c_void;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(TOKEN_TYPE),
    FileNotFound,
    VariableParseError,
    IntegerParseError
}

pub fn gen_ast(context: *const c_void) -> Result<Program, ParseError> {
    let mut my_program = Program::new();

    // Get the first token
    let mut cur_tok = get_token_safe(context);

    while cur_tok.tok_type != TOKEN_TYPE::EOF_TOK {

        match cur_tok.tok_type {
            TOKEN_TYPE::VAR => {
                let result = parse_assign(&mut cur_tok, context)?;
                my_program.program.statements.push(result);
            },
            TOKEN_TYPE::IF => {
                let result = parse_if(&mut cur_tok, context)?;
                my_program.program.statements.push(result);
            },
            TOKEN_TYPE::REPEAT => {
                let result = parse_repeat(&mut cur_tok, context)?;
                my_program.program.statements.push(result);
            },
            TOKEN_TYPE::OUTPUT => {
                let result = parse_output(&mut cur_tok, context)?;
                my_program.program.statements.push(result);
            },
            _ => {
                lexer::close_file_safe(context);
                return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
            }
        }
    }

    return Ok(my_program);
}

pub fn get_file_context(filename: &str) -> Option<*const c_void> {
    open_file_safe(filename)
}

pub fn close_file_from_context(context: *const c_void) {
    close_file_safe(context);
}

fn parse_block(cur_tok: &mut token, context: *const c_void) -> Result<language::Block, ParseError> {
    let mut my_block = language::Block{statements: Vec::new()};

    while cur_tok.tok_type != TOKEN_TYPE::RBRA {

        match cur_tok.tok_type {
            TOKEN_TYPE::VAR => {
                let result = parse_assign(cur_tok, context)?;
                my_block.statements.push(result);
            },
            TOKEN_TYPE::IF => {
                let result = parse_if(cur_tok, context)?;
                my_block.statements.push(result);
            },
            TOKEN_TYPE::REPEAT => {
                let result = parse_repeat(cur_tok, context)?;
                my_block.statements.push(result);
            },
            TOKEN_TYPE::OUTPUT => {
                let result = parse_output(cur_tok, context)?;
                my_block.statements.push(result);
            },
            _ => {
                lexer::close_file_safe(context);
                return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
            }
        }
    }

    return Ok(my_block);
}

fn parse_output(cur_tok: &mut token, context: *const c_void) -> Result<language::Statement, ParseError> {
    consume_token(cur_tok, TOKEN_TYPE::OUTPUT, context)?;

    let to_output = parse_expression(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::SC, context)?;

    return Ok(language::Statement::OutputStatement { to_output: to_output })
}

fn parse_if(cur_tok: &mut token, context: *const c_void) -> Result<language::Statement, ParseError> {
    consume_token(cur_tok, TOKEN_TYPE::IF, context)?;
    consume_token(cur_tok, TOKEN_TYPE::LPAR, context)?;
    
    let if_cond = parse_expression(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::RPAR, context)?;
    consume_token(cur_tok, TOKEN_TYPE::LBRA, context)?;

    let if_body = parse_block(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::RBRA, context)?;

    let mut else_if_vec = Vec::new();
    while cur_tok.tok_type == TOKEN_TYPE::ELSEIF {
        consume_token(cur_tok, TOKEN_TYPE::ELSEIF, context)?;
        consume_token(cur_tok, TOKEN_TYPE::LPAR, context)?;

        let else_if_cond = parse_expression(cur_tok, context)?;

        consume_token(cur_tok, TOKEN_TYPE::RPAR, context)?;
        consume_token(cur_tok, TOKEN_TYPE::LBRA, context)?;

        let else_if_body = parse_block(cur_tok, context)?;

        consume_token(cur_tok, TOKEN_TYPE::RBRA, context)?;

        else_if_vec.push((else_if_cond, else_if_body));
    }

    let else_body;

    if cur_tok.tok_type == TOKEN_TYPE::ELSE {
        consume_token(cur_tok, TOKEN_TYPE::ELSE, context)?;
        consume_token(cur_tok, TOKEN_TYPE::LBRA, context)?;

        else_body = parse_block(cur_tok, context)?;

        consume_token(cur_tok, TOKEN_TYPE::RBRA, context)?;
    } else {
        else_body = Block{statements: Vec::new()}
    }

    return Ok(language::Statement::IfStatement { 
        condition: if_cond, 
        body: if_body, 
        else_if: else_if_vec, 
        else_body: else_body
    });

}

fn parse_repeat(cur_tok: &mut token, context: *const c_void) -> Result<language::Statement, ParseError> {
    // This check should be unnessesary but whatever
    consume_token(cur_tok, TOKEN_TYPE::REPEAT, context)?; // REPEAT
    consume_token(cur_tok, TOKEN_TYPE::LPAR, context)?; // (

    let times = parse_expression(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::RPAR, context)?; // )
    consume_token(cur_tok, TOKEN_TYPE::LBRA, context)?; // {

    let block = parse_block(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::RBRA, context)?; // }

    return Ok(language::Statement::RepeatStatement { times: times, body: block })
}

fn parse_assign(cur_tok: &mut token, context: *const c_void) -> Result<language::Statement, ParseError> {
    // This check should be unnessesary but whatever
    if !(cur_tok.tok_type == TOKEN_TYPE::VAR) {
        return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
    }

    let name_result = lexer::val_to_str(&cur_tok.val);
    let name;

    match name_result {
        None => return Err(ParseError::VariableParseError),
        Some(n) => name = n
    }

    *cur_tok = get_token_safe(context); // Consume the variable

    consume_token(cur_tok, TOKEN_TYPE::ASSIGN, context)?;

    // Follow set of expression
    if !(cur_tok.tok_type == TOKEN_TYPE::VAR ||
         cur_tok.tok_type == TOKEN_TYPE::INT_LIT) {
            return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
    }

    let result = parse_expression(cur_tok, context)?;

    consume_token(cur_tok, TOKEN_TYPE::SC, context)?;

    return Ok(language::Statement::AssignStatement { var: name, exp: result });
}

fn parse_expression(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs: language::Expression;

    if cur_tok.tok_type == TOKEN_TYPE::VAR {
        let tok_val = lexer::val_to_str(&cur_tok.val);
        let name;
        match tok_val {
            None => return Err(ParseError::VariableParseError),
            Some(n) => name = n
        }

        lhs = language::Expression::Var(name);

    } else if cur_tok.tok_type == TOKEN_TYPE::INT_LIT {
        let tok_val = lexer::val_to_str(&cur_tok.val);
        let int_str;
        match tok_val {
            None => return Err(ParseError::IntegerParseError),
            Some(s) => int_str = s
        }

        let tok_val = int_str.parse::<i32>();
        let int_val;
        match tok_val {
            Err(_e) => return Err(ParseError::IntegerParseError),
            Ok(i) => int_val = i
        }

        lhs = language::Expression::Val(int_val);
    } else {
        return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
    }

    *cur_tok = get_token_safe(context);
    
    if cur_tok.tok_type == TOKEN_TYPE::SC || cur_tok.tok_type == TOKEN_TYPE::RPAR {
        return Ok(lhs);
    }

    let op;
    match cur_tok.tok_type {
        TOKEN_TYPE::PLUS => op = language::Op::Add,
        TOKEN_TYPE::MINUS => op = language::Op::Sub,
        TOKEN_TYPE::EQ => op = language::Op::Eq,
        _ => return Err(ParseError::UnexpectedToken(cur_tok.tok_type))
    }
    *cur_tok = get_token_safe(context); // Consume the operation

    let rhs = parse_expression(cur_tok, context)?;

    return Ok(language::Expression::BinOp(op, Box::new(lhs), Box::new(rhs)));
}

fn consume_token(cur_tok: &mut token, expected: TOKEN_TYPE, context: *const c_void) -> Result<(), ParseError> {
    if !(cur_tok.tok_type == expected) {
        return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
    }
    *cur_tok = get_token_safe(context);
    return Ok(());
}