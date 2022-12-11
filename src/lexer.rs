
/* Lexer class */
pub struct Lexer {
    text: String,
    cc: u64
}

/* Initialize a lexer */
pub fn new(text: String) -> Lexer {
    return Lexer {text: text, cc: 0};
}

/* Lexer functions */
impl Lexer {
    pub fn tokenize(&self) -> Vec<token::Token> {
        // Initialize a list of the tokens
        let tokens: Vec<token::Token> = Vec::new();

        // Tokenize the text

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