// Lexical Analysis

pub fn lexer(code: &str) {
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
        }
        x += 1;
    }
    println!("tokens: \n {:?}", tokens);
}

#[derive(Debug)]
enum TokenType {
    Echo(),       // echo function
    LeftParen(),  // left parenthesis
    RightParen(), // right parenthesis
    String(),     // string literal
    NewLine(),    // newline character
    EoF(),        // end of file
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
    index: usize,
}
