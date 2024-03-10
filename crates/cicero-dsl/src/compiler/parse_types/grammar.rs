use chumsky::input::{Stream as TokenStream, ValueInput};
use chumsky::prelude::*;

use super::ast::*;
use super::lexer::Token;
use crate::types::MarkdownString;

fn ident_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, String, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    // NB: don't remove let binding, otherwise `labelled` method can't resolve
    let ident = select! {
        Token::Ident(ident) => ident.to_string(),
    };

    ident.labelled("identifier")
}

fn struct_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Struct, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    comment_parser()
        .or_not()
        .then_ignore(just(Token::STRUCT))
        .then(ident_parser().then(just(Token::Colon).ignore_then(ident_parser()).or_not()))
        .then(
            field_parser()
                .separated_by(just(Token::Comma))
                .collect::<Vec<_>>(),
        )
        .then(
            method_parser()
                .separated_by(just(Token::Comma))
                .collect::<Vec<_>>(),
        )
        .map(|(((comment, (name, parent)), fields), methods)| {
            Struct {
                name,
                comment,
                fields,
                parent,
                methods,
            }
        })
}

fn field_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Field, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    comment_parser()
        .then(ident_parser())
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|((comment, name), ty)| Field { name, comment, ty })
}

fn comment_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, MarkdownString, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    let comment = select! {
        Token::DocComment(doc) => doc.replace("/// ", ""),
    };

    comment.labelled("comment")
}

fn method_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Method, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    just(Token::FN)
        .ignore_then(ident_parser())
        .then(expr_parser().delimited_by(just(Token::LBrace), just(Token::RBrace)))
        .map(|(name, body)| Method { name, body })
}

// TODO
fn expr_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    just(Token::Asterisk).map(|_| Expr::Todo)
}

fn enum_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Enum, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    comment_parser()
        .or_not()
        .then_ignore(just(Token::ENUM))
        .then(ident_parser())
        .then(
            enum_variant_parser()
                .separated_by(just(Token::Comma))
                .collect::<Vec<_>>(),
        )
        .then(
            method_parser()
                .separated_by(just(Token::Comma))
                .collect::<Vec<_>>(),
        )
        .map(|(((comment, name), variants), methods)| {
            Enum {
                name,
                comment,
                variants,
                methods,
            }
        })
}

fn enum_variant_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, EnumVariant, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    comment_parser()
        .then(ident_parser())
        .then(
            type_parser()
                .delimited_by(just(Token::LParen), just(Token::RParen))
                .or_not(),
        )
        .map(|((comment, name), field)| {
            EnumVariant {
                name,
                comment,
                field,
            }
        })
}

fn variable_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Variable, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    comment_parser()
        .then(ident_parser())
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|((comment, name), ty)| Variable { name, comment, ty })
}

fn type_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Type, extra::Err<Rich<'a, Token<'a>>>> + Copy + Clone {
    ident_parser()
        .then(just(Token::QuestionMark).or_not())
        .map(|(name, required)| {
            Type {
                name,
                required: required.is_some(),
            }
        })
}
