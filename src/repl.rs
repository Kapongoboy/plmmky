use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::io::Write;

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error flushing stdout: {}", e);
                break;
            }
        }

        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        if input.trim() == "exit()" {
            break;
        }

        let mut lexer = Lexer::new(&input, true, None);

        let mut tok = lexer.next_token();

        while tok.ttype != TokenKind::EOF {
            println!("{:?}", tok);
            tok = lexer.next_token();
        }
    }
}
