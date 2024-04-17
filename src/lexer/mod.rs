enum TokenType {
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBace,
    Function,
    Let,
}

pub struct Token {
    type: TokenType,
    literal: String,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token{
        let token_type = match self.ch {
            b'=' => TokenType::Assign,
            b'+' => TokenType::Plus,
            b'(' => TokenType::LParen,
            b')' => TokenType::RParen,
            b'{' => TokenType::LBrace,
            b'}' => TokenType::RBrace,
            b',' => TokenType::Comma,
            b';' => TokenType::Semicolon,
            0 => TokenType::EOF,
            _ => TokenType::Ident,
        };
        self.read_char();
        return Token {
            TokenType: todo!(),
            literal: todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        const INPUT: &str = r"=+(){},;";
        let lexer = Lexer::new(INPUT);
        let expected = vec![
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: PLUS.to_string(),
                literal: "+".to_string(),
            },
            Token {
                token_type: LPAREN.to_string(),
                literal: "(".to_string(),
            },
            Token {
                token_type: RPAREN.to_string(),
                literal: ")".to_string(),
            },
            Token {
                token_type: LBRACE.to_string(),
                literal: "{".to_string(),
            },
            Token {
                token_type: RBRACE.to_string(),
                literal: "}".to_string(),
            },
            Token {
                token_type: COMMA.to_string(),
                literal: ",".to_string(),
            },
            Token {
                token_type: SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            Token {
                token_type: EOF.to_string(),
                literal: "".to_string(),
            },
        ];
        for e in expected.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, e.token_type);
            assert_eq!(tok.literal, e.literal);
        }
    }
}
