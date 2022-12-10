

/* Token class  */
pub struct Token {
    pub type_ : String,
    pub data : i8,
}

/* Token initializer  */
pub fn new(type_ : String, data: Option<i8>) -> Token {
    let mut token = Token(type_, data.unwrap_or(0));
    return token;
}

/* Token functions */
pub impl Token {
    fn repr(&self) -> String {

    }
}