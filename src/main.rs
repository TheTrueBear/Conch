
/* Modules */
mod lexer;

/* Uses */
use std::env;

/* Run the code */
fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();

    // Get if it is a file or the shell
    if args.len() == 1 {
        println!("Shell");
    } else {
        println!("File");
    }

    // Make the text
    let text = "1 + 2".to_string();

    // Create the lexer
    let mut lexer = lexer::new(text);
    let tokens = lexer.tokenize();
    println!("{}", tokens.len());
}
