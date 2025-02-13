pub mod lexer;

pub fn compile(code: &str){
    println!("Compiling...");
    lexer::lexer(code);
}