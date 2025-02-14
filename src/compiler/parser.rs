use std::process::exit;

use crate::compiler::lexer::char_position;
use crate::compiler::lexer::Token;
use crate::compiler::lexer::TokenType;

pub fn parser(tokens: Vec<Token>, code: &str) -> Vec<OpCode> {
    // Intermediate instructions (IR)
    let mut ir: Vec<OpCode> = Vec::new();

    let mut x = 0;
    while x < tokens.len() {
        if tokens[x].token_type == TokenType::Echo() && x + 4 < tokens.len() {
            if tokens[x + 1].token_type == TokenType::LeftParen()
                && tokens[x + 2].token_type == TokenType::String()
                && tokens[x + 3].token_type == TokenType::RightParen()
                && tokens[x + 4].token_type == TokenType::NewLine()
                || tokens[x + 4].token_type == TokenType::EoF()
            {
                ir.push(OpCode::Echo(tokens[x + 2].value.clone()));
                x += 4;
            } else {
                let (line, char_at) = char_position(tokens[x].index, code);
                println!("Error[{line}:{char_at}]: Invalid Syntax on line {line} at {char_at}");
                exit(1);
            }
        } else if tokens[x].token_type == TokenType::NewLine()
            || tokens[x].token_type == TokenType::EoF()
        {
            x += 1;
            continue;
        } else {
            let (line, char_at) = char_position(tokens[x].index, code);
            println!("Error[{line}:{char_at}]: Invalid Syntax on line {line} at {char_at}");
            exit(1);
        }
        x += 1;
    }

    ir
}

// IR instructions
#[derive(Debug)]
pub enum OpCode {
    Echo(String),
}
