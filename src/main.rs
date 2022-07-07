use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let context_result = parser::get_file_context(&(args[1]));

    let context;
    match context_result {
        None => panic!("Could not open file: {:?}", &(args[1])),
        Some(c) => context = c
    }

    let parse_result = parser::gen_ast(context);

    parser::close_file_from_context(context);

    let ast;
    match parse_result {
        Err(e) => panic!("PARSING FAILED :(: {:?}", e),
        Ok(a) => ast = a
    }

    let result;
    match interpreter::interpret(ast) {
        Err(e) => panic!("Interpreting Failed :( : {:?}", e),
        Ok(r) => result = r
    }



    for item in result.iter() {
        println!("{}", item);
    }
}