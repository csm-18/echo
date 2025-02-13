// Lexical Analysis

use std::process::exit;

pub fn lexer(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut x = 0;
    while x < code.len() {
        if &code[x..x + 1] == "#" {
            // ignore comments and get newline(if present)
            let mut found_newline = false;
            let mut y = x;
            while y < code.len() {
                if &code[y..y + 1] == "\n" {
                    found_newline = true;
                    break;
                }
                y += 1;
            }
            if found_newline {
                let temp = Token {
                    token_type: TokenType::NewLine(),
                    value: String::from("\n"),
                    index: y,
                };
                tokens.push(temp);
                x = y;
            } else {
                break;
            }
        } else if &code[x..x + 1] == "(" {
            tokens.push(Token {
                token_type: TokenType::LeftParen(),
                value: String::from("("),
                index: x,
            });
        } else if &code[x..x + 1] == ")" {
            tokens.push(Token {
                token_type: TokenType::RightParen(),
                value: ")".to_string(),
                index: x,
            });
        } else if &code[x..x + 1] == "\n" {
            tokens.push(Token {
                token_type: TokenType::NewLine(),
                value: "\n".to_string(),
                index: x,
            });
        } else if &code[x..x + 1] == "\"" {
            //get string literal

            let mut closing_quote = false;
            let mut temp = String::from("\"");
            let mut y = x + 1;
            while y < code.len() {
                temp.push_str(&code[y..y + 1]);
                if &code[y..y + 1] == "\"" {
                    closing_quote = true;
                    break;
                }
                y += 1;
            }
            if closing_quote {
                tokens.push(Token {
                    token_type: TokenType::String(),
                    value: temp,
                    index: x,
                });
                x = y;
            } else {
                let (line_number, char_at) = char_position(x, code);
                println!(
                    "Error[{line_number},{char_at}]: Expected closing quote(\") on line {} at {}",
                    line_number, char_at
                );
                exit(1);
            }
        } else if is_alpha_numeric(&code[x..x + 1]) {
            let mut temp = String::new();
            let mut y = x;
            while y < code.len() && is_alpha_numeric(&code[y..y + 1]) {
                temp.push_str(&code[y..y + 1]);
                y += 1;
            }

            if temp.contains("echo") {
                tokens.push(Token {
                    token_type: TokenType::Echo(),
                    value: "echo".to_string(),
                    index: x,
                });
                x = y - 1;
            } else {
                let (line_number, char_at) = char_position(x, code);
                println!(
                    "Error[{line_number},{char_at}]: Invalid token on line {} at {}",
                    line_number, char_at
                );
                exit(1);
            }
        } else if &code[x..x + 1] == " " {
            // ignore whitespace
            x += 1;
            continue;
        }else {
            let (line_number, char_at) = char_position(x, code);
            println!(
                "Error[{line_number},{char_at}]: Invalid token on line {} at {}",
                line_number, char_at
            );
            exit(1);
        }
        x += 1;
    }
    
    tokens
}

#[derive(Debug)]
pub enum TokenType {
    Echo(),       // echo function
    LeftParen(),  // left parenthesis
    RightParen(), // right parenthesis
    String(),     // string literal
    NewLine(),    // newline character
    EoF(),        // end of file
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
    index: usize,
}

// get position of a char in a string
fn char_position(char_index: usize, code: &str) -> (usize, usize) {
    let mut line_number = 1;
    let mut previous_newline_index = 0;

    let mut x = 0;
    while x < code.len() {
        if &code[x..x + 1] == "\n" {
            previous_newline_index = x;
            line_number += 1;
        }
        x += 1;
    }
    let char_position_in_line = char_index - previous_newline_index;

    (line_number, char_position_in_line)
}

// check if a string is alphanumeric
fn is_alpha_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric())
}
