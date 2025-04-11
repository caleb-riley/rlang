use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Identifer,
    Number,
    String,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Equals,
    Period,

    Plus,
    Minus,
    Star,
    Slash,
    LessThan,
    GreaterThan,

    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    FnKeyword,
    StructKeyword,
    LetKeyword,
    ReturnKeyword,
    IfKeyword,

    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

impl Token {
    fn new(text: String, kind: TokenKind) -> Self {
        Self { text, kind }
    }
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    position: usize,
    length: usize,
    symbols: HashMap<char, TokenKind>,
    keywords: HashMap<String, TokenKind>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let symbols = {
            let mut symbols = HashMap::new();

            symbols.insert('(', TokenKind::LeftParen);
            symbols.insert(')', TokenKind::RightParen);
            symbols.insert('{', TokenKind::LeftBrace);
            symbols.insert('}', TokenKind::RightBrace);
            symbols.insert(',', TokenKind::Comma);
            symbols.insert(';', TokenKind::Semicolon);
            symbols.insert(':', TokenKind::Colon);
            symbols.insert('+', TokenKind::Plus);
            symbols.insert('-', TokenKind::Minus);
            symbols.insert('*', TokenKind::Star);
            symbols.insert('/', TokenKind::Slash);
            symbols.insert('=', TokenKind::Equals);
            symbols.insert('<', TokenKind::LessThan);
            symbols.insert('>', TokenKind::GreaterThan);
            symbols.insert('.', TokenKind::Period);

            symbols
        };

        let keywords = {
            let mut keywords = HashMap::new();

            keywords.insert("fn".to_owned(), TokenKind::FnKeyword);
            keywords.insert("struct".to_owned(), TokenKind::StructKeyword);
            keywords.insert("let".to_owned(), TokenKind::LetKeyword);
            keywords.insert("return".to_owned(), TokenKind::ReturnKeyword);
            keywords.insert("true".to_owned(), TokenKind::TrueKeyword);
            keywords.insert("false".to_owned(), TokenKind::FalseKeyword);
            keywords.insert("null".to_owned(), TokenKind::NullKeyword);
            keywords.insert("if".to_owned(), TokenKind::IfKeyword);

            keywords
        };

        Self {
            source: source.chars().collect::<Vec<char>>(),
            tokens: vec![],
            position: 0,
            length: source.len(),
            symbols,
            keywords,
        }
    }

    fn current(&self) -> Option<char> {
        if self.position < self.length {
            Some(self.source[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn scan_string(&mut self) {
        let start = self.position;

        self.advance();

        while let Some(current) = self.current() {
            if current == '"' {
                break;
            }

            self.advance();
        }

        self.advance();

        let text = self
            .source
            .iter()
            .skip(start)
            .take(self.position - start)
            .collect::<String>();

        self.tokens.push(Token::new(text, TokenKind::String))
    }

    fn scan_number(&mut self) {
        let start = self.position;

        while let Some(current) = self.current() {
            if !current.is_ascii_digit() {
                break;
            }

            self.advance();
        }

        let text = self
            .source
            .iter()
            .skip(start)
            .take(self.position - start)
            .collect::<String>();

        self.tokens.push(Token::new(text, TokenKind::Number))
    }

    fn scan_identifier(&mut self) {
        let start = self.position;

        while let Some(current) = self.current() {
            if !current.is_ascii_alphabetic() && current != '_' {
                break;
            }

            self.advance();
        }

        let text = self
            .source
            .iter()
            .skip(start)
            .take(self.position - start)
            .collect::<String>();

        let kind = self.keywords.get(&text).unwrap_or(&TokenKind::Identifer);

        self.tokens.push(Token::new(text, *kind));
    }

    fn skip_whitespace(&mut self) {
        while let Some(current) = self.current() {
            if !current.is_ascii_whitespace() {
                break;
            }

            self.advance();
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while let Some(current) = self.current() {
            if current.is_ascii_digit() {
                self.scan_number();
            } else if current.is_ascii_alphabetic() || current == '_' {
                self.scan_identifier();
            } else if current.is_ascii_whitespace() {
                self.skip_whitespace();
            } else if self.symbols.contains_key(&current) {
                self.tokens.push(Token::new(
                    self.source[self.position..self.position + 1]
                        .iter()
                        .collect::<String>(),
                    self.symbols[&current],
                ));
                self.advance();
            } else if current == '"' {
                self.scan_string();
            } else {
                panic!("Invalid char: {}", current);
            }
        }

        self.tokens
            .push(Token::new("\0".to_owned(), TokenKind::EndOfFile));

        self.tokens
    }
}
