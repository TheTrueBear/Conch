
/* Modules */
mod lexer;

/* Uses */
use std::env;

/* Run the code */
fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
