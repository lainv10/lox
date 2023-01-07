use std::fmt::Display;
use thiserror::Error;

type Number = f32;

#[derive(Debug)]
pub enum Token {
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
    Star,
    Slash,

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

    /// End of file
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{id}"),
            Token::String(s) => {
                write!(f, "\"{s}\"")
            }
            Token::Number(n) => {
                write!(f, "{n}")
            }

            _ => write!(
                f,
                "{}",
                match self {
                    Token::LeftParen => "(",
                    Token::RightParen => ")",
                    Token::LeftBrace => "{",
                    Token::RightBrace => "}",
                    Token::Comma => ",",
                    Token::Dot => ".",
                    Token::Minus => "-",
                    Token::Plus => "+",
                    Token::SemiColon => ";",
                    Token::Star => "*",
                    Token::Slash => "/",
                    Token::Bang => "!",
                    Token::BangEqual => "!=",
                    Token::Equal => "=",
                    Token::EqualEqual => "==",
                    Token::Greater => ">",
                    Token::GreaterEqual => ">=",
                    Token::Less => "<",
                    Token::LessEqual => "<=",
                    Token::And => "and",
                    Token::Class => "class",
                    Token::Else => "else",
                    Token::False => "false",
                    Token::Fun => "fun",
                    Token::For => "for",
                    Token::If => "if",
                    Token::Nil => "nil",
                    Token::Or => "or",
                    Token::Print => "print",
                    Token::Return => "return",
                    Token::Super => "super",
                    Token::This => "this",
                    Token::True => "true",
                    Token::Var => "var",
                    Token::While => "while",
                    Token::Eof => "",
                    _ => unreachable!(),
                }
            ),
        }
    }
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unknown token at line {0}")]
    UnknownToken(usize),

    #[error("Unterminated string starting at line {0}")]
    UnterminatedString(usize),

    #[error("Invalid number literal at line {0}")]
    InvalidNumber(usize),
}

pub struct Scanner {
    /// Source code string.
    src: String,

    /// List of chars representing the source string.
    chars: Vec<char>,

    /// An index into the source string that indicates the current position of the `Scanner`.
    current: usize,

    /// The start position of the token the `Scanner` is currently processing.
    start: usize,

    /// The current line number in the source code the `Scanner` is processing.
    line: usize,

    /// Errors collected while scanning the source.
    errors: Vec<ScannerError>,
}

impl Scanner {
    /// Create a new `Scanner` from a source code string.
    pub fn new(src: String) -> Self {
        let chars = src.chars().collect();
        Self {
            src,
            chars,
            current: 0,
            start: 0,
            line: 1,
            errors: Default::default(),
        }
    }

    /// Scan the source code and produce a list of tokens.
    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // loop through all tokens in the source
        while !self.at_end() {
            self.start = self.current;

            // add token
            match self.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => (),
                Err(e) => self.errors.push(e),
            }

            // set position to the start of the next token
            self.advance();
        }

        tokens.push(Token::Eof);

        tokens
    }

    /// Get the token starting at the current position of the `Scanner`.
    ///
    /// Returns a `ScannerError` if there was an error while scanning the token,
    ///
    /// The result will be `None` if the token should not be scanned (whitespace, newlines, etc).
    ///
    /// Assuming the next token is valid, the `Scanner` position will
    /// be moved to the last character of the token.
    /// Whitespace and newlines will be skipped over
    fn scan_token(&mut self) -> Result<Option<Token>, ScannerError> {
        match self.peek() {
            // single character tokens
            '(' => Ok(Some(Token::LeftParen)),
            ')' => Ok(Some(Token::RightParen)),
            '{' => Ok(Some(Token::LeftBrace)),
            '}' => Ok(Some(Token::RightBrace)),
            ',' => Ok(Some(Token::Comma)),
            '.' => Ok(Some(Token::Dot)),
            '-' => Ok(Some(Token::Minus)),
            '+' => Ok(Some(Token::Plus)),
            ';' => Ok(Some(Token::SemiColon)),
            '*' => Ok(Some(Token::Star)),
            '/' => Ok(self.comment_or_slash()),

            // handle two character tokens
            '!' | '=' | '<' | '>' => Ok(self.two_char_token()),

            // skip over whitespace
            ' ' | '\r' | '\t' => Ok(None),

            // increment line count on \n
            '\n' => {
                self.line += 1;
                Ok(None)
            }

            '"' => self.string().map(Some),

            c if c.is_ascii_digit() => self.number().map(Some),

            c if c.is_alphabetic() => Ok(Some(self.identifier())),

            _ => Err(ScannerError::UnknownToken(self.line)),
        }
    }

    /// Handle identifier tokens. Should be called when the `Scanner`
    /// is processing an alphanumeric character as the start of a new token.
    ///
    /// Returns a token representing a keyword if the identifier is a keyword.
    /// Otherwise, it returns an identifier token.
    ///
    /// This will advance the `Scanner` position to the end of the identifier token.
    fn identifier(&mut self) -> Token {
        while self.peek_next().is_alphanumeric() {
            self.advance();
        }
        let identifier = &self.src[self.start..self.current + 1];
        match identifier {
            "and" => Token::And,
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "fun" => Token::Fun,
            "for" => Token::For,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "var" => Token::Var,
            "while" => Token::While,
            _ => Token::Identifier(identifier.to_string()),
        }
    }

    /// Handle number tokens. Should be called when the `Scanner` is
    /// processing a digit 0-9.
    ///
    /// Returns the appropriate number token, or a `ScannerError` if the number
    /// could not be parsed.
    ///
    /// This will advance the `Scanner` position to the end of the number token.
    fn number(&mut self) -> Result<Token, ScannerError> {
        while self.peek_next().is_ascii_digit() {
            self.advance();
        }
        // We're at the end of the first part of the number,
        // but there may be a fractional component to the literal,
        // so we look for that too.
        // Have to check if the char after the period is a digit too,
        // since we don't allow literals like '1234.'
        if self.peek_next() == '.' {
            // advance cursor position to '.'
            self.advance();

            // case where number is something like '1234.'
            if !self.peek_next().is_ascii_digit() {
                return Err(ScannerError::InvalidNumber(self.line));
            }

            while self.peek_next().is_ascii_digit() {
                self.advance();
            }
        }

        let num = self.src[self.start..self.current + 1]
            .parse::<f32>()
            .unwrap();
        Ok(Token::Number(num))
    }

    /// Handle literal string tokens. This function should be called when the `Scanner` is
    /// currently on a quote character.
    ///
    /// Returns the appropriate token,
    /// else a `ScannerError` if there's an error (e.g. unterminated string).
    ///
    /// This will advance the `Scanner` position to the end of the string
    /// literal token (at the end quote character).
    fn string(&mut self) -> Result<Token, ScannerError> {
        let mut delta_lines = 0;
        while (self.peek_next() != '"') && !self.at_end() {
            if self.peek() == '\n' {
                delta_lines += 1;
            }
            self.advance();
        }

        // advance position to ending quote
        self.advance();

        if self.at_end() {
            return Err(ScannerError::UnterminatedString(self.line));
        }

        self.line += delta_lines;

        // we don't want the quotes to be part of the rust string representation
        let str_literal = self.src[self.start + 1..self.current].to_string();
        Ok(Token::String(str_literal))
    }

    /// Handle tokens that are two characters long. This function can be called
    /// if the scanner is currently on a character that either resolves to a single
    /// token, or is the start of a two character token, e.g. '!', '>'.
    ///
    /// Returns the appropriate token for the one or two character token.
    /// Returns `None` if none of the one or two character tokens can be matched.
    ///
    /// This function will advance the `Scanner` position to the end of the token (if found).
    fn two_char_token(&mut self) -> Option<Token> {
        let first_char = self.peek();

        let equal_next = self.match_next('=');
        if equal_next {
            self.advance();
        }

        match (first_char, equal_next) {
            ('!', true) => Some(Token::BangEqual),
            ('!', false) => Some(Token::Bang),

            ('=', true) => Some(Token::EqualEqual),
            ('=', false) => Some(Token::Equal),

            ('>', true) => Some(Token::GreaterEqual),
            ('>', false) => Some(Token::Greater),

            ('<', true) => Some(Token::LessEqual),
            ('<', false) => Some(Token::Less),

            _ => None,
        }
    }

    /// Handle a token that starts with a '/'; it may
    /// be the start of a comment or a single slash.
    ///
    /// Returns the appropriate `Token` if is a single slash,
    /// and `None` if it is a comment.
    ///
    /// This function will advance the `Scanner` position to the end of the token / comment.
    fn comment_or_slash(&mut self) -> Option<Token> {
        // check if next token is a comment
        if self.match_next('/') {
            while (self.peek_next() != '\n') && !self.at_end() {
                self.advance();
            }
            None
        } else {
            Some(Token::Slash)
        }
    }

    /// Advance the `Scanner`'s current position by one.
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
