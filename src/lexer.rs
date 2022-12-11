
/* Lexer class */
pub struct Lexer {
    text: String,
    cc: String,
    i: u64
}

/* Initialize a lexer */
pub fn new(text: String) -> Lexer {
    return Lexer {text: text, cc: "".to_string(), i: 0};
}

/* Lexer functions */
impl Lexer {

    // Set the character of the lexer that it is on
    fn set_char(&mut self) {
        self.cc = "1".to_string();
    }

    // Advance to the next character
    fn advance(&mut self) {
        self.i += 1;
        self.set_char();
    }

    // Function to tokenize a string
    pub fn tokenize(&mut self) -> Vec<token::Token> {
        // Initialize a list of the tokens
        let tokens: Vec<token::Token> = Vec::new();

        // Tokenize the text
        while self.cc != String::new() {

        }

        // Return the tokens
        return tokens;
    }
}

/* Token module */
mod token {
    /* Token class  */
    pub struct Token {
        pub type_ : i8,
        pub data : String,
    }

    /* Token initializer  */
    pub fn new(type_ : i8, data: String) -> Token {
        let token = Token {
            type_: type_,
            data: data
        };
        return token;
    }

    /* Token functions */
    impl Token {
        pub fn repr(&self) -> String {
            return format!("{} : {}", self.type_, self.data).to_string();
        }
    }
}