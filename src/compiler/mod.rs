pub mod interpreter;
pub mod lexer;
pub mod parser;
pub fn compile(code: &str) {
    let tokens = lexer::lexer(code);
    let ir = parser::parser(tokens, code);
    interpreter::exec(ir);
}
