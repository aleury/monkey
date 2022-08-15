#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Illegal,

    // Identifiers and literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Let,
    Function,
}
