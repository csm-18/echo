// Lexical Analysis


pub fn lexer(code: &str) {
    let mut tokens: Vec<Token> = Vec::new();

    
    println!("tokens: \n {:?}", tokens);
}


#[derive(Debug)]
enum TokenType {
    Echo(),        // echo function
    LeftParen(),   // left parenthesis
    RightParen(),  // right parenthesis
    String(),      // string literal
    NewLine(),     // newline character
    EoF(),         // end of file
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
    index: usize,
}
