pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    let user = match std::env::var("USER") {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error getting user: {}, using fallback name", e);
            String::from("friend")
        }
    };

    if !given_file_arg() {
        println!("Hello {}! This is the Monkey programming language!", user);
        repl::start();
    } else {
        eprintln!("File argument not yet supported, functionality coming soon!");
    }
}

fn given_file_arg() -> bool {
    let args: Vec<String> = std::env::args().collect();
    args.len() > 1
}
