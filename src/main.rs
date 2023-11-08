use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let context = parser::get_file_context(&(args[1]))
                                        .expect(&format!("Could not open file: {:?}", &(args[1])));

    
    let parse_result = parser::gen_ast(context);

    parser::close_file_from_context(context);

    let ast = match parse_result {
        Err(e) => panic!("PARSING FAILED :(: {:?}", e),
        Ok(a) => a
    };

    let result = match interpreter::interpret(ast) {
        Err(e) => panic!("Interpreting Failed :( : {:?}", e),
        Ok(r) => r
    };

    for item in result.iter() {
        println!("{}", item);
    }
}