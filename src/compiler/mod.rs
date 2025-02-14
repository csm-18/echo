pub mod lexer;
pub mod parser;
pub fn compile(code: &str) {
    println!("Compiling...");
    let tokens = lexer::lexer(code);
    let ir = parser::parser(tokens, code);
    println!("{:?}", ir);
}
