
/* A module with all the staticants for this */
// Base types
pub static VOID: i8 = 0;
pub static NUM: i8 = 1;
pub static STR: i8 = 2;

// Operators
pub static PLUS: i8 = 3;
pub static MINUS: i8 = 4;
pub static MUL: i8 = 5;
pub static DIV: i8 = 6;
pub static POW: i8 = 7;
pub static MOD: i8 = 8;
pub static FLDIV: i8 = 9;

// Surrounding things
pub static LPAREN: i8 = 10;
pub static RPAREN: i8 = 11;
pub static LBRACE: i8 = 12;
pub static RBRACE: i8 = 13;
pub static LSQUARE: i8 = 14;
pub static RSQUARE: i8 = 15;

// Equals
pub static EQ: i8 = 16;

// Ifs
pub static EE: i8 = 17;
pub static NE: i8 = 18;
pub static GT: i8 = 19;
pub static GTE: i8 = 20;
pub static LT: i8 = 21;
pub static LTE: i8 = 22;

// Keywords
pub static KEYWORDS: [String; 8] = [
    "let".to_string(),
    "fun".to_string(),
    "global".to_string(),
    "for".to_string(),
    "to".to_string(),
    "is".to_string(),
    "not".to_string(),
    "and".to_string()
];