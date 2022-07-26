use language::*;
use lexer::*;

use std::os::raw::c_void;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(TOKEN_TYPE),
    UnknownOperator(TOKEN_TYPE),
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

fn rvalor(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    
    match cur_tok.tok_type {
        TOKEN_TYPE::OR => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, TOKEN_TYPE::OR, context)?;

            let rhs = rvaland(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvalor(cur_tok, context, result)
        },
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        },
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvaland(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvaleq(cur_tok, context)?;
    rvaland_p(cur_tok, context, lhs)
}

fn rvaland_p(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    
    match cur_tok.tok_type {
        TOKEN_TYPE::AND => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, TOKEN_TYPE::AND, context)?;

            let rhs = rvaleq(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvaland_p(cur_tok, context, result)
        },
        TOKEN_TYPE::OR |
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        },
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvaleq(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvalcomp(cur_tok, context)?;
    rvaleq_p(cur_tok, context, lhs)
}

fn rvaleq_p(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    
    match cur_tok.tok_type {
        TOKEN_TYPE::EQ |
        TOKEN_TYPE::NEQ => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, cur_tok.tok_type, context)?;

            let rhs = rvalcomp(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvaleq_p(cur_tok, context, result)
        },
        TOKEN_TYPE::AND |
        TOKEN_TYPE::OR |
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        },
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvalcomp(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvaladd(cur_tok, context)?;
    rvalcomp_p(cur_tok, context, lhs)
}

fn rvalcomp_p(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    match cur_tok.tok_type {
        TOKEN_TYPE::LE | 
        TOKEN_TYPE::LT | 
        TOKEN_TYPE::GE | 
        TOKEN_TYPE::GT => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, cur_tok.tok_type, context)?;

            let rhs = rvaladd(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvalcomp_p(cur_tok, context, result)
        },
        TOKEN_TYPE::EQ |
        TOKEN_TYPE::NEQ |
        TOKEN_TYPE::AND |
        TOKEN_TYPE::OR |
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        }
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvaladd(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvalmult(cur_tok, context)?;
    rvaladd_p(cur_tok, context, lhs)
}

fn rvaladd_p(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    match cur_tok.tok_type {
        TOKEN_TYPE::PLUS | 
        TOKEN_TYPE::MINUS => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, cur_tok.tok_type, context)?;

            let rhs = rvalmult(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvaladd_p(cur_tok, context, result)
        },
        TOKEN_TYPE::LE | 
        TOKEN_TYPE::LT | 
        TOKEN_TYPE::GE | 
        TOKEN_TYPE::GT |
        TOKEN_TYPE::EQ |
        TOKEN_TYPE::NEQ |
        TOKEN_TYPE::AND |
        TOKEN_TYPE::OR |
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        }
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvalmult(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvallit(cur_tok, context)?;
    rvalmult_p(cur_tok, context, lhs)
}

fn rvalmult_p(cur_tok: &mut token, context: *const c_void, lhs: language::Expression) -> Result<language::Expression, ParseError> {
    match cur_tok.tok_type {
        TOKEN_TYPE::ASTERIX | 
        TOKEN_TYPE::DIV |
        TOKEN_TYPE::MOD => {
            let this_token = cur_tok.clone();
            consume_token(cur_tok, cur_tok.tok_type, context)?;

            let rhs = rvallit(cur_tok, context)?;
            let result = language::Expression::BinOp(token_to_op(this_token.tok_type)?, Box::new(lhs), Box::new(rhs));
            rvalmult_p(cur_tok, context, result)
        },
        TOKEN_TYPE::PLUS | 
        TOKEN_TYPE::MINUS |
        TOKEN_TYPE::LE | 
        TOKEN_TYPE::LT | 
        TOKEN_TYPE::GE | 
        TOKEN_TYPE::GT |
        TOKEN_TYPE::EQ |
        TOKEN_TYPE::NEQ |
        TOKEN_TYPE::AND |
        TOKEN_TYPE::OR |
        TOKEN_TYPE::SC |
        TOKEN_TYPE::RPAR => {
            Ok(lhs)
        }
        _ => {
            Err(ParseError::UnexpectedToken(cur_tok.tok_type))
        }
    }
}

fn rvallit(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    match cur_tok.tok_type {
        TOKEN_TYPE::VAR => {
            match lexer::val_to_str(&cur_tok.val) {
                Some(name) => {
                    consume_token(cur_tok, TOKEN_TYPE::VAR, context)?;
                    Ok(language::Expression::Var(name))
                },
                None => Err(ParseError::VariableParseError)
            }
        },
        TOKEN_TYPE::INT_LIT => {
            match lexer::val_to_str(&cur_tok.val).map(|s| s.parse::<i32>()) {
                Some(Ok(i)) => {
                    consume_token(cur_tok, TOKEN_TYPE::INT_LIT, context)?;
                    Ok(language::Expression::Val(i))
                },
                _ => Err(ParseError::IntegerParseError)
            }
        }
        _ => Err(ParseError::UnexpectedToken(cur_tok.tok_type))
    }
}

fn parse_expression(cur_tok: &mut token, context: *const c_void) -> Result<language::Expression, ParseError> {
    let lhs = rvaland(cur_tok, context)?;
    return rvalor(cur_tok, context, lhs);
}

fn consume_token(cur_tok: &mut token, expected: TOKEN_TYPE, context: *const c_void) -> Result<(), ParseError> {
    if !(cur_tok.tok_type == expected) {
        return Err(ParseError::UnexpectedToken(cur_tok.tok_type));
    }
    *cur_tok = get_token_safe(context);
    return Ok(());
}

/// Consumes the token without checking what it is for a parse error
/// Only to be used if the token could be on of a set of options
fn consume_token_unchecked(cur_tok: &mut token, context: *const c_void) {
    *cur_tok = get_token_safe(context);
}

fn token_to_op(tok: TOKEN_TYPE) -> Result<Op, ParseError> {
    match tok {
        TOKEN_TYPE::PLUS => Ok(Op::Add),
        TOKEN_TYPE::MINUS => Ok(Op::Sub),
        TOKEN_TYPE::ASTERIX => Ok(Op::Multiply),
        TOKEN_TYPE::DIV => Ok(Op::Divide),
        TOKEN_TYPE::MOD => Ok(Op::Remainder),
        TOKEN_TYPE::AND => Ok(Op::And),
        TOKEN_TYPE::OR => Ok(Op::Or),
        TOKEN_TYPE::EQ => Ok(Op::Equal),
        TOKEN_TYPE::NEQ => Ok(Op::NotEqual),
        TOKEN_TYPE::LE => Ok(Op::LessThanOrEqual),
        TOKEN_TYPE::LT => Ok(Op::LessThan),
        TOKEN_TYPE::GE => Ok(Op::GreaterThanOrEqual),
        TOKEN_TYPE::GT => Ok(Op::GreaterThan),
        _ => Err(ParseError::UnknownOperator(tok))
    }
}