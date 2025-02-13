pub mod lexer;

pub fn compile(code: &str) {
    println!("Compiling...");
    let tokens = lexer::lexer(code);
    println!("tokens: \n {:?}", tokens);

}
