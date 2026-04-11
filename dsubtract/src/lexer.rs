#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Let,
    If,
    Else,
    While,
    Assign,
    End,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: i64,
    pub lexeme: String,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.peek() {
            if c == '/' && self.input.get(self.pos + 1) == Some(&'/') {
                self.consume();
                self.consume();
                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        break;
                    }
                    self.consume();
                }
                return self.next_token();
            }
            if c.is_ascii_digit() {
                let mut num_str = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(self.consume().unwrap());
                    } else {
                        break;
                    }
                }
                let value = num_str.parse().unwrap_or(0);
                return Token {
                    token_type: TokenType::Number,
                    value,
                    lexeme: num_str,
                };
            }
            if c.is_ascii_alphabetic() {
                let mut lex = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_alphanumeric() {
                        lex.push(self.consume().unwrap());
                    } else {
                        break;
                    }
                }
                let token_type = match lex.as_str() {
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "while" => TokenType::While,
                    _ => TokenType::Identifier,
                };
                return Token {
                    token_type,
                    value: 0,
                    lexeme: lex,
                };
            }
            self.consume();
            let token_type = match c {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Star,
                '/' => TokenType::Slash,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                '{' => TokenType::LBrace,
                '}' => TokenType::RBrace,
                '=' => TokenType::Assign,
                _ => panic!("Unexpected char"),
            };
            Token {
                token_type,
                value: 0,
                lexeme: c.to_string(),
            }
        } else {
            Token {
                token_type: TokenType::End,
                value: 0,
                lexeme: String::new(),
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }
}
