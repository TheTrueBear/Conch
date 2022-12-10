
/* Modules */
mod token;

/* Run the code */
fn main() {
    let tok = token::new(1, "Bear".to_string());
    println!("{}", tok.repr());
}
