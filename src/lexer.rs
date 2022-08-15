use crate::tokens::Token;

pub struct Lexer<'a> {
    /// source code being tokenized
    input: &'a str,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = match self.ch {
            '=' => Some(Token::Assign),
            '+' => Some(Token::Plus),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semicolon),
            '\x00' => Some(Token::EOF),
            _ => None,
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = self
            .input
            .chars()
            .nth(self.read_position)
            .unwrap_or_default();
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::tokens::Token;

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new("=+(){},;");

        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        for token in expected_tokens {
            let result = lexer.next_token();
            let expected = Some(token);
            assert_eq!(result, expected);
        }
    }
}
