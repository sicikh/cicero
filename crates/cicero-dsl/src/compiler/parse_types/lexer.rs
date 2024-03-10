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
    // ==== KEYWORDS ====
    #[regex("abort")]
    STRUCT,
    #[regex("enum")]
    ENUM,
}
