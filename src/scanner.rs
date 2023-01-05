type Number = f32;

#[derive(Debug)]
pub enum TokenKind {
    // Punctuation / Single character token
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(Number),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    pub lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}

pub struct Scanner {
    /// Source code string.
    src: String,

    /// List of chars represnting the source string.
    chars: Vec<char>,

    /// An index into the source string that indicates the current position of the `Scanner`.
    current: usize,
}

impl Scanner {
    /// Create a new `Scanner` from a source string representing Lox code.
    pub fn new(src: String) -> Self {
        let chars = src.chars().collect();
        Self {
            src,
            chars,
            current: 0,
        }
    }

    /// Scan the source code and produce a list of tokens.
    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut start;
        let mut line = 1;

        // advance
        while !self.at_end() {
            start = self.current;

            let first_char = self.peek();
            self.advance();
            let kind = match first_char {
                // single character tokens
                '(' => Some(TokenKind::LeftParen),
                ')' => Some(TokenKind::RightParen),
                '{' => Some(TokenKind::LeftBrace),
                '}' => Some(TokenKind::RightBrace),
                ',' => Some(TokenKind::Comma),
                '.' => Some(TokenKind::Dot),
                '-' => Some(TokenKind::Minus),
                '+' => Some(TokenKind::Plus),
                ';' => Some(TokenKind::SemiColon),
                '/' => {
                    // check if token is a comment
                    if self.match_next('/') {
                        self.advance();
                        while (self.peek() != '\n') && !self.at_end() {
                            self.advance();
                        }
                        None
                    } else {
                        Some(TokenKind::Slash)
                    }
                }
                '*' => Some(TokenKind::Star),

                // one or two character tokens
                // check the following character to determine the token kind
                '!' | '=' | '<' | '>' => {
                    let equal_next = self.match_next('=');
                    if equal_next {
                        self.advance();
                    }
                    match (first_char, equal_next) {
                        ('!', true) => Some(TokenKind::BangEqual),
                        ('!', false) => Some(TokenKind::Bang),

                        ('=', true) => Some(TokenKind::EqualEqual),
                        ('=', false) => Some(TokenKind::Equal),

                        ('>', true) => Some(TokenKind::GreaterEqual),
                        ('>', false) => Some(TokenKind::Greater),

                        ('<', true) => Some(TokenKind::LessEqual),
                        ('<', false) => Some(TokenKind::Less),

                        _ => unreachable!(),
                    }
                }

                // skip over newlines and whitespace
                ' ' | '\r' | '\t' => None,
                '\n' => {
                    line += 1;
                    None
                }

                '"' => {
                    let mut delta_lines = 0;
                    while (self.peek() != '"') && (!self.at_end()) {
                        if self.peek() == '\n' {
                            delta_lines += 1;
                        }
                        self.advance();
                    }

                    if self.at_end() {
                        eprintln!("[Error]: Unterminated string starting at line {line}.");
                    }

                    line += delta_lines;

                    // go past the closing "
                    self.advance();

                    let str_literal = self.src[start + 1..self.current - 1].to_string();
                    Some(TokenKind::String(str_literal))
                }

                _ => {
                    // check if the char is a digit
                    if first_char.is_ascii_digit() {
                        while self.peek().is_ascii_digit() {
                            self.advance();
                        }

                        // we're at the end of the first part of the number,
                        // but there may be a fractional component to the literal,
                        // so we look for that too.
                        // Have to check if the char after the period is a digit too, since
                        // we don't allow literals like "1234."
                        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
                            // go past the '.'
                            self.advance();

                            while self.peek().is_ascii_digit() {
                                self.advance();
                            }
                        }

                        if let Ok(num) = self.src[start..self.current].parse::<f32>() {
                            Some(TokenKind::Number(num))
                        } else {
                            eprintln!("[Interpreter Error]: Failed to parse number literal at line {line}");
                            None
                        }
                    // check if char is the start of an identifier
                    } else if first_char.is_alphabetic() {
                        while self.peek().is_alphanumeric() {
                            self.advance();
                        }

                        let identifier = &self.src[start..self.current];
                        match identifier {
                            "and" => Some(TokenKind::And),
                            "class" => Some(TokenKind::Class),
                            "else" => Some(TokenKind::Else),
                            "false" => Some(TokenKind::False),
                            "fun" => Some(TokenKind::Fun),
                            "for" => Some(TokenKind::For),
                            "if" => Some(TokenKind::If),
                            "nil" => Some(TokenKind::Nil),
                            "or" => Some(TokenKind::Or),
                            "print" => Some(TokenKind::Print),
                            "return" => Some(TokenKind::Return),
                            "super" => Some(TokenKind::Super),
                            "this" => Some(TokenKind::This),
                            "true" => Some(TokenKind::True),
                            "var" => Some(TokenKind::Var),
                            "while" => Some(TokenKind::While),
                            _ => Some(TokenKind::Identifier(identifier.to_string())),
                        }
                    } else {
                        eprintln!("[Error]: Invalid token '{first_char}'");
                        None
                    }
                }
            };

            if let Some(kind) = kind {
                let lexeme = &self.src[start..self.current];
                tokens.push(Token::new(kind, lexeme.into(), line))
            }
        }

        tokens.push(Token::new(TokenKind::Eof, String::new(), line));

        tokens
    }

    /// Advance the `Scanner`'s current position.
    #[inline]
    fn advance(&mut self) {
        self.current += 1;
    }

    /// Return `true` if the `Scanner` position is past the end of the source code.
    #[inline]
    fn at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    /// Return the character in `self.src` at the current position.
    #[inline]
    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    /// Return the character in `self.src` one after the current position.
    #[inline]
    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    /// Inspect the character in `self.src` after the current `Scanner` position.
    /// Returns `true` if it matches the given character, `false` otherwise.
    #[inline]
    fn match_next(&self, to_match: char) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek_next() == to_match
        }
    }
}
