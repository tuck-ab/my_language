use crate::*;

use ::language::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

// pub fn empty_program() -> Program {
//     Program::new()
// }


#[test]
fn test_assign() {
    let mut test_memory = Memory::new();

    pub_assign_test("Test1", 5, &mut test_memory);
    assert_eq!(test_memory.mem[&String::from("Test1")], 5);

    pub_assign_test("Test2", 1, &mut test_memory);
    assert_eq!(test_memory.mem[&String::from("Test1")], 5);
    assert_eq!(test_memory.mem[&String::from("Test2")], 1);

    pub_assign_test("Test1", 2, &mut test_memory);
    assert_ne!(test_memory.mem[&String::from("Test1")], 5);
    assert_eq!(test_memory.mem[&String::from("Test1")], 2);
    assert_eq!(test_memory.mem[&String::from("Test2")], 1);
}

#[test]
fn test_access() {
    let mut test_memory = Memory::new();

    pub_assign_test("Test1", 1, &mut test_memory);
    pub_assign_test("Test2", 2, &mut test_memory);

    assert_eq!(pub_access_test("Test1", &mut test_memory), Ok(1));
    assert_eq!(pub_access_test("Test2", &mut test_memory), Ok(2));

    assert_eq!(pub_access_test("Test3", &mut test_memory), 
        Err(
            ErrorType::UninitialisedMemory(
                String::from("Variable \"Test3\" has not been assigned"))
            )
    );
    
    pub_assign_test("Test3", 3, &mut test_memory);
    assert_eq!(pub_access_test("Test3", &mut test_memory), Ok(3));
}

#[test]
fn test_binary_operations() {
    let mut test_memory = Memory::new();

    // Test some adding
    assert_eq!(pub_eval_binop_test(&Op::Add, &Expression::Val(5), &Expression::Val(3), &mut test_memory).unwrap(), 8);
    assert_eq!(pub_eval_binop_test(&Op::Add, &Expression::Val(1), &Expression::Val(2), &mut test_memory).unwrap(), 3);

    // Test some subtraction
    assert_eq!(pub_eval_binop_test(&Op::Sub, &Expression::Val(5), &Expression::Val(3), &mut test_memory).unwrap(), 2);
    assert_eq!(pub_eval_binop_test(&Op::Sub, &Expression::Val(3), &Expression::Val(6), &mut test_memory).unwrap(), -3);

    // Test some equalities
    assert_eq!(pub_eval_binop_test(&Op::Eq, &Expression::Val(5), &Expression::Val(3), &mut test_memory).unwrap(), 0);
    assert_eq!(pub_eval_binop_test(&Op::Eq, &Expression::Val(3), &Expression::Val(3), &mut test_memory).unwrap(), 1);
}

#[test]
fn test_evaluate_expressions() {
    let mut test_memory = Memory::new();

    // Accessing literals
    assert_eq!(pub_eval_test(&Expression::Val(5), &mut test_memory).unwrap(), 5);
    assert_eq!(pub_eval_test(&Expression::Val(-35), &mut test_memory).unwrap(), -35);


    // Accessing variables
    pub_assign_test("Test1", 1, &mut test_memory);
    pub_assign_test("Test2", -2, &mut test_memory);

    assert_eq!(pub_eval_test(&Expression::Var(String::from("Test1")), &mut test_memory).unwrap(), 1);
    assert_eq!(pub_eval_test(&Expression::Var(String::from("Test2")), &mut test_memory).unwrap(), -2);

    assert!(pub_eval_test(&Expression::Var(String::from("Test3")), &mut test_memory).is_err());

    // Binary expressions
    let expression1 = Expression::BinOp(
        Op::Add,
        Box::new(Expression::Val(5)),
        Box::new(Expression::Val(3))
    );
    assert_eq!(pub_eval_test(&expression1, &mut test_memory).unwrap(), 8);

    pub_assign_test("Test1", 10, &mut test_memory);
    let expression2 = Expression::BinOp(
        Op::Sub,
        Box::new(Expression::Var(String::from("Test1"))),
        Box::new(expression1)
    );
    assert_eq!(pub_eval_test(&expression2, &mut test_memory).unwrap(), 2);
}

#[test]
fn test_for_loop() {
    let mut test_memory = Memory::new();
    let mut output_vec = Vec::new();

    let loop_code = Block{
        statements: vec![
            Statement::AssignStatement { 
                var: String::from("x"), 
                exp: Expression::BinOp(
                    Op::Add, 
                    Box::new(Expression::Var(String::from("x"))), 
                    Box::new(Expression::Val(1))
                ) 
            }
        ]
    };

    let code = Block{
        statements: vec![
            Statement::AssignStatement { 
                var: String::from("x"), 
                exp: Expression::Val(0) 
            },
            Statement::RepeatStatement { 
                times: Expression::Val(10), 
                body: loop_code
            }
        ]
    };

    match pub_run_block(&code, &mut test_memory,&mut output_vec) {
        Err(e) => panic!("Error during interpret: {:?}", e),
        Ok(()) => ()
    }

    assert_eq!(pub_access_test("x", &mut test_memory), Ok(10))
}

#[test]
fn test_if_stmt() {
    let mut test_memory = Memory::new();
    let mut output_vec = Vec::new();

    let code = Block{
        statements:vec![
            Statement::IfStatement { 
                condition: Expression::BinOp(
                    Op::Eq,
                    Box::new(Expression::Var(String::from("x"))),
                    Box::new(Expression::Val(0))
                ), 
                body: Block{
                    statements:vec![
                        Statement::AssignStatement { 
                            var: String::from("result"), 
                            exp: Expression::Val(0) 
                        }
                    ]
                }, 
                else_if: vec![
                    (Expression::BinOp(
                        Op::Eq,
                        Box::new(Expression::Var(String::from("x"))),
                        Box::new(Expression::Val(1))
                    ), 
                    Block{
                        statements:vec![
                            Statement::AssignStatement { 
                                var: String::from("result"), 
                                exp: Expression::Val(1) 
                            }
                        ]
                    }),
                    (Expression::BinOp(
                        Op::Eq,
                        Box::new(Expression::Var(String::from("x2"))),
                        Box::new(Expression::Val(2))
                    ), 
                    Block{
                        statements:vec![
                        Statement::AssignStatement { 
                            var: String::from("result"), 
                            exp: Expression::Val(2) 
                        }
                    ]
                    })
                ], 
                else_body: Block{
                    statements:vec![
                        Statement::AssignStatement { 
                            var: String::from("result"), 
                            exp: Expression::Val(4) 
                        }
                    ]
                }, 
            }
        ]
    };

    // raw if
    pub_assign_test("x", 0, &mut test_memory);
    match pub_run_block(&code, &mut test_memory, &mut output_vec) {
        Err(e) => panic!("Error during interpret: {:?}", e),
        Ok(()) => ()
    }
    assert_eq!(pub_access_test("result", &mut test_memory), Ok(0));
    assert_ne!(pub_access_test("result", &mut test_memory), Ok(1));

    // Else if
    pub_assign_test("x", 1, &mut test_memory);
    pub_assign_test("x2", 2, &mut test_memory);
    // First and not second
    match pub_run_block(&code, &mut test_memory, &mut output_vec) {
        Err(e) => panic!("Error during interpret: {:?}", e),
        Ok(()) => ()
    }
    assert_eq!(pub_access_test("result", &mut test_memory), Ok(1));

    // Second
    pub_assign_test("x", 5, &mut test_memory);
    match pub_run_block(&code, &mut test_memory, &mut output_vec) {
        Err(e) => panic!("Error during interpret: {:?}", e),
        Ok(()) => ()
    }
    assert_eq!(pub_access_test("result", &mut test_memory), Ok(2));

    // Else
    pub_assign_test("x2", 5, &mut test_memory);
    match pub_run_block(&code, &mut test_memory, &mut output_vec) {
        Err(e) => panic!("Error during interpret: {:?}", e),
        Ok(()) => ()
    }
    assert_eq!(pub_access_test("x", &mut test_memory), Ok(5));
    assert_eq!(pub_access_test("x2", &mut test_memory), Ok(5));
    assert_eq!(pub_access_test("result", &mut test_memory), Ok(4));


}