#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use ::language::*;


#[derive(PartialEq, Debug)]
pub enum ErrorType {
    NegativeRepeateError,
    UninitialisedMemory(String),
    NotImplimented
}

pub struct Memory {
    pub mem: HashMap<String, i32>
}

impl Memory {
    pub fn new() -> Self {
        Self{mem: HashMap::new()}
    }
}

pub fn interpret(program: Program) -> Result<Vec<i32>, ErrorType> {
    // Initialise some memory for variables
    let mut memory = Memory::new();

    // Initialise output buffer
    let mut output_vec = Vec::new();

    run_block(&program.program, &mut memory, &mut output_vec)?;

    return Ok(output_vec);
}

#[cfg(test)]
pub fn pub_run_block(block: &Block, memory: &mut Memory, output_vec: &mut Vec<i32>) -> Result<(), ErrorType> {
    run_block(block, memory, output_vec)
}

fn run_block(block: &Block, memory: &mut Memory, output_vec: &mut Vec<i32>) -> Result<(), ErrorType> {
    // Loop through the program statement by statement
    for stmt in (*block).statements.as_slice() {
        match stmt {
            // If the statement is an assign statment
            Statement::AssignStatement{
                var, 
                exp
            } => {
                let eval = eval_exp(exp, memory)?;
                assign(&var, eval, memory);
            },

            Statement::IfStatement{
                condition, 
                body, 
                else_if, 
                else_body
            } => {
                let eval = eval_exp(condition, memory)?;
                let mut has_run: bool = false;

                if eval != 0 {
                    run_block(body, memory, output_vec)?;
                    has_run = true;
                } else {
                    for (exp, block) in else_if {
                        let eval = eval_exp(exp, memory)?;

                        if eval != 0 {
                            run_block(block, memory, output_vec)?;
                            has_run = true;
                            break;
                        }
                    }
                }

                if !has_run {
                    run_block(else_body, memory, output_vec)?;
                }
            },

            Statement::RepeatStatement{
                times, 
                body
            } => {
                let eval = eval_exp(&times, memory)?;

                if eval < 0 {
                    return Err(ErrorType::NegativeRepeateError)
                } else {
                    for _i in 0..eval {
                        run_block(&body, memory, output_vec)?;
                    }
                }
            },

            Statement::OutputStatement { to_output } => {
                let eval = eval_exp(to_output, memory)?;
                output_vec.push(eval);
            }
        };
    }

    return Ok(());
}

#[cfg(test)]
pub fn pub_assign_test(var: &str, val: i32, memory: &mut Memory) {
    assign(var, val, memory);
}

fn assign(var: &str, val: i32, memory: &mut Memory) {
    let entry = memory.mem.entry(String::from(var)).or_insert(0);
    *entry = val;
}

#[cfg(test)]
pub fn pub_access_test(var: &str, memory: &mut Memory) -> Result<i32, ErrorType> {
    access(var, memory)
}

fn access(var: &str, memory: &mut Memory) -> Result<i32, ErrorType> {
    let e = memory.mem.entry(String::from(var));

    match e {
        Entry::Occupied(
            entry
        ) => return Ok(*(entry.get())),
        Entry::Vacant(
            _
        ) => return Err(
            ErrorType::UninitialisedMemory(
                format!("Variable {:?} has not been assigned", var)
            )
        )
    }
}

#[cfg(test)]
pub fn pub_eval_test(exp: &Expression, memory: &mut Memory) -> Result<i32, ErrorType> {
    eval_exp(exp, memory)
}

fn eval_exp(exp: &Expression, memory: &mut Memory) -> Result<i32, ErrorType> {
    match exp {
        Expression::Val(num) => return Ok(*num),
        Expression::Var(var) => return access(&var, memory),
        Expression::BinOp(
            op,
            lhs,
            rhs
        ) => return eval_bin_op(op, &*lhs, &*rhs, memory)
    }
}

#[cfg(test)]
pub fn pub_eval_binop_test(op: &Op, lhs: &Expression, rhs: &Expression, memory: &mut Memory) -> Result<i32, ErrorType> {
    eval_bin_op(op, lhs, rhs, memory)
}

fn eval_bin_op(op: &Op, lhs: &Expression, rhs: &Expression, memory: &mut Memory) -> Result<i32, ErrorType> {
    let lhs_eval = eval_exp(&lhs, memory)?;

    let rhs_eval = eval_exp(&rhs, memory)?;

    // At this point the errors should be dealt with so the evals will be i32
    // which means they can simply be unwrapped

    //TODO overflow add etc.
    match op {
        Op::Add => return Ok(lhs_eval + rhs_eval),
        Op::Sub => return Ok(lhs_eval - rhs_eval),
        Op::Equal => return Ok((lhs_eval == rhs_eval) as i32),
        _ => return Err(ErrorType::NotImplimented)
    }
}
