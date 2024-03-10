use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use logos::{Lexer, Logos};

#[inline]
fn number<'src>(lex: &'_ mut Lexer<'src, Token<'src>>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Token<'src> {
    #[regex(r"\d+", number)]
    Number(i64),
    #[regex("[_\\p{L}\\p{M}][_\\p{L}\\p{M}0-9]*")]
    Ident(&'src str),
    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"", strip_quotes)]
    DoubleQuotedString(&'src str),
    #[token(">=")]
    GtEq,
    #[token("<=")]
    LtEq,
    #[regex("(!=|<>)")]
    Neq,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("=")]
    Eq,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    // ==== KEYWORDS ====
    #[regex("abort")]
    STRUCT,
    #[regex("enum")]
    ENUM,
    #[regex("match")]
    MATCH,
}
