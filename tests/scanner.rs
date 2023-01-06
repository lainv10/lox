//! Tests for the public interface of [`lox::scanner::Scanner`].

use lox::scanner::{Scanner, Token, TokenKind};

/// Helper function to get tokens from given source string.
#[inline]
fn scan(src: impl Into<String>) -> Vec<Token> {
    Scanner::new(src.into()).scan()
}

/// Check a list of tokens against an expected list of token kinds and lexemes
fn check(expected: Vec<(TokenKind, &str)>, actual: Vec<Token>) {
    assert_eq!(expected.len(), actual.len());
    for (token, (_kind, lexeme)) in actual.iter().zip(expected.iter()) {
        assert!(matches!(&token.kind, _kind));
        assert_eq!(&token.lexeme, lexeme);
    }
}

/// Empty source string should return no tokens except EOF.
#[test]
fn empty_src() {
    let tokens = scan("");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].kind, TokenKind::Eof));
}

/// Whitespace should be ignored by scanner
#[test]
fn empty_whitespace() {
    let tokens = scan("    \n\r\t\n\n\n\r\t      \n\t");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].kind, TokenKind::Eof));
}

#[test]
fn var_assignment_num() {
    let expected = vec![
        (TokenKind::Var, "var"),
        (TokenKind::Identifier("foo".into()), "foo"),
        (TokenKind::Equal, "="),
        (TokenKind::Number(2.0), "2"),
        (TokenKind::Eof, ""),
    ];

    check(expected, scan("var foo = 2"));
}

#[test]
fn two_character_operator() {
    let expected = vec![
        (TokenKind::Identifier("a".into()), "a"),
        (TokenKind::BangEqual, "!="),
        (TokenKind::Identifier("b".into()), "b"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Identifier("b".into()), "b"),
        (TokenKind::LessEqual, "<="),
        (TokenKind::Identifier("c".into()), "c"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Identifier("c".into()), "c"),
        (TokenKind::GreaterEqual, ">="),
        (TokenKind::Identifier("d".into()), "d"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Identifier("a".into()), "a"),
        (TokenKind::EqualEqual, "=="),
        (TokenKind::Identifier("a".into()), "a"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Eof, ""),
    ];

    check(expected, scan("a != b; b <= c; c >= d; a == a;"));
}

#[test]
fn single_line_string() {
    let expected = vec![
        (
            TokenKind::String("this is a string".into()),
            "\"this is a string\"",
        ),
        (TokenKind::Eof, ""),
    ];

    check(expected, scan("\"this is a string\""))
}

#[test]
fn single_line_comment() {
    let tokens = scan("// this is a comment");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].kind, TokenKind::Eof));
}

#[test]
fn ending_comment() {
    let tokens = scan("var foo = 2 // this is a comment");
    // scanning for the expression before comment is handled by another test
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens.iter().last().unwrap().kind, TokenKind::Eof))
}
