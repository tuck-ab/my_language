extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/lexer.c");
    cc::Build::new()
        .file("src/lexer.c")
        .compile("lexer.a");
}