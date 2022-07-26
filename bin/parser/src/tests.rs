use crate::*;

#[test]
fn just_assign() {
    let context = get_file_context("test/justassign.xa").expect("Could not open file");

    let parse_result = gen_ast(context);
    let ast;
    match parse_result {
        Err(e) => panic!("AST generation returned Err: {:?}", e),
        Ok(a) => ast = a
    }

    let predicted = language::Program{
        program: Block { statements: vec![
            language::Statement::AssignStatement { 
                var: String::from("variable"), 
                exp: language::Expression::Val(4) 
            },

            language::Statement::AssignStatement { 
                var: String::from("variabletwo"), 
                exp: language::Expression::BinOp(
                    language::Op::Add,
                    Box::new(Expression::Val(3)),
                    Box::new(Expression::Val(9))
                ) 
            }
        ] }
    };

    assert_eq!(ast, predicted);
}

#[test]
fn just_repeat() {
    let context = get_file_context("test/justrepeat.xa").expect("Could not open file");

    let parse_result = gen_ast(context);
    let ast;
    match parse_result {
        Err(e) => panic!("AST generation returned Err: {:?}", e),
        Ok(a) => ast = a
    }

    let predicted = language::Program{
        program: Block { statements: vec![
            language::Statement::RepeatStatement { 
                times: language::Expression::Val(4), 
                body: Block { statements: vec![
                    language::Statement::AssignStatement { 
                        var: String::from("x"), 
                        exp: language::Expression::Val(5) 
                    }
                ] }
            },

            language::Statement::RepeatStatement { 
                times: language::Expression::BinOp(
                    language::Op::Add,
                    Box::new(Expression::Val(4)),
                    Box::new(Expression::Val(5))
                ) , 
                body: Block { statements: vec![
                    language::Statement::AssignStatement { 
                        var: String::from("y"), 
                        exp: language::Expression::Val(2) 
                    }
                ] }   
            }
        ] }
    };

    assert_eq!(ast, predicted);
}

#[test]
fn just_if() {
    let context = get_file_context("test/justif.xa").expect("Could not open file");

    let parse_result = gen_ast(context);
    let ast;
    match parse_result {
        Err(e) => panic!("AST generation returned Err: {:?}", e),
        Ok(a) => ast = a
    }

    let predicted = language::Program{
        program: Block { statements: vec![
            language::Statement::IfStatement { 
                condition: language::Expression::Val(1), 

                body: Block { statements: vec![
                    language::Statement::AssignStatement { 
                        var: String::from("x"), 
                        exp: language::Expression::Val(5) 
                    }
                ] },

                else_if: vec![
                    (
                        Expression::BinOp(
                            language::Op::Equal,
                            Box::new(Expression::Val(3)),
                            Box::new(Expression::Val(3))
                        ) , 

                        Block { statements: vec![
                            language::Statement::AssignStatement { 
                                var: String::from("y"), 
                                exp: language::Expression::Val(2) 
                            }
                        ] },
                    ) , (
                        Expression::BinOp(
                            language::Op::Sub,
                            Box::new(Expression::Val(1)),
                            Box::new(Expression::Val(1))
                        ) , 

                        Block { statements: vec![
                            language::Statement::AssignStatement { 
                                var: String::from("z"), 
                                exp: language::Expression::Val(5) 
                            }
                        ] },
                    )
                ],

                else_body: Block { statements: vec![
                            language::Statement::AssignStatement { 
                                var: String::from("abc"), 
                                exp: language::Expression::Val(0) 
                            }
                        ] }
            },

            language::Statement::IfStatement { 
                condition: language::Expression::Val(0), 

                body: Block { statements: vec![
                    language::Statement::AssignStatement { 
                        var: String::from("abcd"), 
                        exp: language::Expression::Val(1) 
                    }
                ] },

                else_if: vec![],

                else_body: Block { statements: vec![] }
            },
        ] }
    };

    assert_eq!(ast, predicted);
}