

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