use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,

    Illegal,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirlybrace,
    Rsquirlybrace,
    Minus,

    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    Identifier(String),
    Int(usize),
}

// Rust-PHF is a library to generate efficient lookup tables at compile time using perfect hash functions.
static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "true" => Token::True,
    "false" => Token::False,
    "fn" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
};

// returns optional ref to the next element without consuming it
#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        loop {
            match self.read_char() {
                Some('*') => return Some(Token::Asterisk),
                Some('!') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::NotEqual);
                        }
                    }
                    return Some(Token::Bang);
                }
                Some('/') => return Some(Token::Slash),
                Some('>') => return Some(Token::Gt),
                Some('<') => return Some(Token::Lt),
                Some('-') => return Some(Token::Minus),
                Some('+') => return Some(Token::Plus),
                Some(',') => return Some(Token::Comma),
                Some('=') => {
                    if let Some(c) = self.peek() {
                        if *c == '=' {
                            self.read_char();
                            return Some(Token::Equal);
                        }
                    }
                    return Some(Token::Assign);
                }
                Some(';') => return Some(Token::Semicolon),
                Some('(') => return Some(Token::Lparen),
                Some(')') => return Some(Token::Rparen),
                Some('{') => return Some(Token::Lsquirlybrace),
                Some('}') => return Some(Token::Rsquirlybrace),

                Some(c) if c.is_digit(10) => {
                    let str = self.keep_reading(c, |c| c.is_digit(10));
                    let str = str.into_iter().collect::<String>();
                    return Some(Token::Int(
                        str::parse::<usize>(&str).expect("this should always work"),
                    ));
                }

                Some(c) if c.is_ascii_alphabetic() => {
                    let ident = self.keep_reading(c, |c| c.is_ascii_alphabetic());
                    let ident = ident.into_iter().collect::<String>();

                    // phf::get_entry both the key and the value
                    if let Some((_, v)) = KEYWORDS.get_entry(&ident) {
                        return Some(v.clone());
                    }
                    return Some(Token::Identifier(ident));
                }

                Some(_) => return Some(Token::Illegal),
                _ => return None,
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Lexer<'a> {
        return Lexer {
            chars: code.chars().peekable(),
        };
    }

    // Returns a reference to the next() value without advancing the iterator.
    fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }

    // Advances the iterator and returns the next value.
    fn read_char(&mut self) -> Option<char> {
        return self.chars.next();
    }

    // avoid whitespace
    fn skip_whitespace(&mut self) {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}
    }

    // keep reading until there is no more char
    fn keep_reading(&mut self, c: char, f: impl Fn(&char) -> bool) -> Vec<char> {
        let mut out = vec![c];
        while let Some(c) = self.chars.next_if(&f) {
            out.push(c);
        }

        return out;
    }
}
