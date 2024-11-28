/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::fmt::{Display, Formatter};

use logos::{Lexer, Logos};

#[inline]
#[allow(clippy::unnecessary_wraps)]
fn strip_quotes<'src>(lex: &Lexer<'src, Token<'src>>) -> Result<&'src str, ()> {
    let slice = lex.slice();
    let stripped = &slice[1..slice.len() - 1];
    Ok(stripped)
}

#[inline]
fn number<'src>(lex: &Lexer<'src, Token<'src>>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?;
    Some(n)
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("?")]
    QuestionMark,
    #[regex(r"///[^\n\r]*(?:\*\)|[\n\r])")]
    DocComment(&'src str),
    // ==== KEYWORDS ====
    #[regex("struct")]
    STRUCT,
    #[regex("enum")]
    ENUM,
    #[regex("let")]
    LET,
    // ==== CONTROL TOKENS ====
    // Are not included in the logos (!) lexer output
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,
    #[regex(r"--[^\r\n]*(\r\n|\n)?", logos::skip)]
    Comment,
    Unknown(&'src str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::Ident(ident) => write!(f, "{ident}"),
            Token::DoubleQuotedString(s) => write!(f, "\"{s}\""),
            Token::GtEq => write!(f, ">="),
            Token::LtEq => write!(f, "<="),
            Token::Neq => write!(f, "!="),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::Eq => write!(f, "="),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Period => write!(f, "."),
            Token::QuestionMark => write!(f, "?"),
            Token::DocComment(doc) => write!(f, "/// {doc}"),
            Token::STRUCT => write!(f, "struct"),
            Token::ENUM => write!(f, "enum"),
            Token::LET => write!(f, "let"),
            Token::Whitespace => write!(f, " "),
            Token::Comment => write!(f, "--"),
            Token::Unknown(s) => write!(f, "#{s}#"),
        }
    }
}
