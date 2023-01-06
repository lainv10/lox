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

    Eof,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
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

    /// The start position of the token the `Scanner` is currently processing.
    start: usize,

    /// The current line number in the source code the `Scanner` is processing.
    line: usize,
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
        }
    }

    /// Scan the source code and produce a list of tokens.
    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // loop through all tokens in the source
        while !self.at_end() {
            self.start = self.current;

            // add token
            if let Some(token_kind) = self.scan_token() {
                tokens.push(Token::new(token_kind, self.lexeme(), self.line))
            }

            // set position to the start of the next token
            self.advance();
        }

        tokens.push(Token::new(TokenKind::Eof, String::new(), self.line));

        tokens
    }

    /// Get the token starting at the current position of the `Scanner`.
    ///
    /// Returns `None` if there was an error while scanning the token,
    /// or if the token should not be scanned (e.g. whitespace and newlines).
    ///
    /// Assuming the next token is valid, the `Scanner` position will
    /// be moved to the last character of the token.
    /// Whitespace and newlines will be skipped over
    fn scan_token(&mut self) -> Option<TokenKind> {
        match self.peek() {
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
            '*' => Some(TokenKind::Star),
            '/' => self.comment_or_slash(),

            // handle two character tokens
            '!' | '=' | '<' | '>' => self.two_char_token(),

            // skip over whitespace
            ' ' | '\r' | '\t' => None,

            // increment line count on \n
            '\n' => {
                self.line += 1;
                None
            }

            '"' => self.string(),

            c if c.is_ascii_digit() => self.number(),

            c if c.is_alphabetic() => self.identifier(),

            _ => {
                eprintln!("[Error]: Invalid token '{}'", self.peek());
                None
            }
        }
    }

    /// Handle identifier tokens. Should be called when the `Scanner`
    /// is processing an alphanumeric character as the start of a new token.
    ///
    /// Returns a `TokenKind` represnting a keyword if the identifier is a keyword.
    /// Otherwise, it returns an identifier token.
    ///
    /// This will advance the `Scanner` position to the end of the identifier token.
    fn identifier(&mut self) -> Option<TokenKind> {
        while self.peek_next().is_alphanumeric() {
            self.advance();
        }
        let identifier = &self.src[self.start..self.current + 1];
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
    }

    /// Handle number tokens. Should be called when the `Scanner` is
    /// processing a digit 0-9.
    ///
    /// Returns the appropriate number token, or `None` if the number
    /// could not be parsed.
    ///
    /// This will advance the `Scanner` position to the end of the number token.
    fn number(&mut self) -> Option<TokenKind> {
        while self.peek_next().is_ascii_digit() {
            self.advance();
        }
        // We're at the end of the first part of the number,
        // but there may be a fractional component to the literal,
        // so we look for that too.
        // Have to check if the char after the period is a digit too, since
        // we don't allow literals like '1234.'
        if self.peek_next() == '.' {
            // advance cursor position to '.'
            self.advance();

            // case where number is something like '1234.'
            if !self.peek_next().is_ascii_digit() {
                eprintln!("[Error]: Invalid number literal at line {}", self.line);
            }

            while self.peek_next().is_ascii_digit() {
                self.advance();
            }
        }
        if let Ok(num) = self.src[self.start..self.current + 1].parse::<f32>() {
            Some(TokenKind::Number(num))
        } else {
            eprintln!(
                "[Interpreter Error]: Failed to parse number literal at line {}",
                self.line
            );
            None
        }
    }

    /// Handle literal string tokens. This function should be called when the `Scanner` is
    /// currently on a quote character.
    ///
    /// Returns the appropriate `TokenKind`,
    /// else `None` if there's an error (e.g. unterminated string)
    ///
    /// This will advance the `Scanner` position to the end of the string
    /// literal token (at the end quote character).
    fn string(&mut self) -> Option<TokenKind> {
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
            eprintln!(
                "[Error]: Unterminated string starting at line {}.",
                self.line
            );
            return None;
        }

        self.line += delta_lines;

        // we don't want the quotes to be part of the rust string representation
        let str_literal = self.src[self.start + 1..self.current].to_string();
        Some(TokenKind::String(str_literal))
    }

    /// Handle tokens that are two characters long. This function can be called
    /// if the scanner is currently on a character that either resolves to a single
    /// token, or is the start of a two character token, e.g. '!', '>'.
    ///
    /// Returns the appropriate `TokenKind` for the one or two character token.
    /// Returns `None` if none of the one or two character tokens can be matched.
    ///
    /// This function will advance the `Scanner` position to the end of the token (if found).
    fn two_char_token(&mut self) -> Option<TokenKind> {
        let first_char = self.peek();
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

            _ => None,
        }
    }

    /// Handle a token that starts with a '/'; it may
    /// be the start of a comment or a single slash.
    ///
    /// Returns the appropriate `TokenKind` if is a single slash,
    /// and `None` if it is a comment.
    ///
    /// This function will advance the `Scanner` position to the end of the token / comment.
    fn comment_or_slash(&mut self) -> Option<TokenKind> {
        // check if next token is a comment
        if self.match_next('/') {
            while (self.peek_next() != '\n') && !self.at_end() {
                self.advance();
            }
            None
        } else {
            Some(TokenKind::Slash)
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

    /// Get the current lexeme defined by the start and current positions of the `Scanner`.
    fn lexeme(&self) -> String {
        self.src[self.start..self.current + 1].to_string()
    }
}
