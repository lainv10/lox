//! Tests for the public interface of [`lox::scanner::Scanner`].

use lox::scanner::{Scanner, Token};

/// Helper function to get tokens from given source string.
#[inline]
fn scan(src: impl Into<String>) -> Vec<Token> {
    Scanner::new(src.into()).scan()
}

/// Check a list of tokens against an expected list of token kinds and lexemes
fn check(expected: Vec<(Token, &str)>, actual: Vec<Token>) {
    assert_eq!(expected.len(), actual.len());
    for (token, (_kind, lexeme)) in actual.iter().zip(expected.iter()) {
        assert!(matches!(&token, _kind));
        assert_eq!(token.to_string(), lexeme.to_string());
    }
}

/// Empty source string should return no tokens except EOF.
#[test]
fn empty_src() {
    let tokens = scan("");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

/// Whitespace should be ignored by scanner
#[test]
fn empty_whitespace() {
    let tokens = scan("    \n\r\t\n\n\n\r\t      \n\t");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

#[test]
fn var_assignment_num() {
    let expected = vec![
        (Token::Var, "var"),
        (Token::Identifier("foo".into()), "foo"),
        (Token::Equal, "="),
        (Token::Number(2.0), "2"),
        (Token::Eof, ""),
    ];

    check(expected, scan("var foo = 2"));
}

#[test]
fn two_character_operator() {
    let expected = vec![
        (Token::Identifier("a".into()), "a"),
        (Token::BangEqual, "!="),
        (Token::Identifier("b".into()), "b"),
        (Token::SemiColon, ";"),
        (Token::Identifier("b".into()), "b"),
        (Token::LessEqual, "<="),
        (Token::Identifier("c".into()), "c"),
        (Token::SemiColon, ";"),
        (Token::Identifier("c".into()), "c"),
        (Token::GreaterEqual, ">="),
        (Token::Identifier("d".into()), "d"),
        (Token::SemiColon, ";"),
        (Token::Identifier("a".into()), "a"),
        (Token::EqualEqual, "=="),
        (Token::Identifier("a".into()), "a"),
        (Token::SemiColon, ";"),
        (Token::Eof, ""),
    ];

    check(expected, scan("a != b; b <= c; c >= d; a == a;"));
}

#[test]
fn single_line_string() {
    let expected = vec![
        (
            Token::String("this is a string".into()),
            "\"this is a string\"",
        ),
        (Token::Eof, ""),
    ];

    check(expected, scan("\"this is a string\""))
}

#[test]
fn single_line_comment() {
    let tokens = scan("// this is a comment");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Eof));
}

#[test]
fn ending_comment() {
    let tokens = scan("var foo = 2 // this is a comment");
    // scanning for the expression before comment is handled by another test
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens.iter().last().unwrap(), Token::Eof))
}
