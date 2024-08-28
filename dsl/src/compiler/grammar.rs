/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use chumsky::input::{SpannedInput, Stream as TokenStream, ValueInput};
use chumsky::prelude::*;
use logos::Logos;

use super::ast::*;
use super::lexer::Token;
use crate::compiler::parse_markdown;
use crate::types::HtmlString;

// FIXME: create normal Error type. For now String is enough
pub fn parse_module(input: &str) -> Result<Module, String> {
    let res: Result<Module, Vec<Rich<'_, Token<'_>>>> =
        module_parser().parse(wrap_lexer(input)).into_result();

    match res {
        Ok(ast) => Ok(ast),
        Err(err) => {
            let err = err
                .into_iter()
                .map(|err| err.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            Err(err)
        },
    }
}

/// module ::= ( struct | enum )* variables
fn module_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Module, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    struct_parser()
        .map(TypeDef::Struct)
        .or(enum_parser().map(TypeDef::Enum))
        .repeated()
        .collect::<Vec<_>>()
        .then(variables_parser())
        .map(|(type_decls, variables)| {
            Module {
                type_defs: type_decls,
                variables,
            }
        })
}

/// ident ::= [a-zA-Z_][a-zA-Z0-9_]*
fn ident_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, String, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    // NB: don't remove let binding, otherwise `labelled` method can't resolve
    let ident = select! {
        Token::Ident(ident) => ident.to_string(),
    };

    ident.labelled("identifier")
}

/// struct ::= [ comment ] 'struct' ident [ ':' ident ] '{' fields '}'
fn struct_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Struct, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    comment_parser()
        .or_not()
        .then_ignore(just(Token::STRUCT))
        .then(ident_parser().then(just(Token::Colon).ignore_then(ident_parser()).or_not()))
        .then(fields_parser().delimited_by(just(Token::LBrace), just(Token::RBrace)))
        .map(|((comment, (name, parent)), fields)| {
            Struct { name, comment, fields, parent }
        })
}

/// fields ::= ( field )*
fn fields_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Vec<Field>, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    field_parser()
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
}

/// field ::= comment ident ':' type
fn field_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Field, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    comment_parser()
        .then(ident_parser())
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|((comment, name), ty)| Field { name, comment, ty })
}

/// comment ::= ( '/// ' [^\n\r] )*
fn comment_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, HtmlString, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    let comment = select! {
        Token::DocComment(doc) => doc.replace("/// ", "").replace("///", ""),
    };

    comment
        .labelled("comment")
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|comments| comments.join("").trim().to_string())
        .map(|markdown| parse_markdown(&markdown))
}

/// enum ::= [ comment ] 'enum' ident '{' enum_variant [ ',' enum_variant ]* [
/// ',' ] '}'
fn enum_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Enum, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    comment_parser()
        .or_not()
        .then_ignore(just(Token::ENUM))
        .then(ident_parser())
        .then(
            enum_variant_parser()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map(|((comment, name), variants)| {
            Enum {
                name,
                comment,
                variants,
            }
        })
}

/// enum_variant ::= comment ident [ '(' type ')' ]
fn enum_variant_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, EnumVariant, extra::Err<Rich<'a, Token<'a>>>> + Copy {
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

/// variables ::= variable ( ';' variable )* [ ';' ]
fn variables_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Vec<Variable>, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    variable_parser()
        .separated_by(just(Token::Semicolon))
        .allow_trailing()
        .at_least(1)
        .collect()
}

/// variable ::= comment 'let' ident ':' type
fn variable_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Variable, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    comment_parser()
        .then_ignore(just(Token::LET))
        .then(ident_parser())
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|((comment, name), ty)| Variable { name, comment, ty })
}

/// type ::= ( ident | '[' ident ']' ) [ '?' ]
fn type_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Type, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    ident_parser()
        .map(|name| (name, false))
        .or(ident_parser()
            .delimited_by(just(Token::LBracket), just(Token::RBracket))
            .map(|name| (name, true)))
        .then(just(Token::QuestionMark).or_not())
        .map(|((name, is_array), required)| {
            Type {
                name,
                is_array,
                is_required: required.is_none(),
            }
        })
}

fn wrap_lexer(
    src: &str,
) -> SpannedInput<Token, SimpleSpan, TokenStream<impl Iterator<Item = (Token, SimpleSpan)>>> {
    let lex_iter = Token::lexer(src).spanned().map(|(tok, span)| {
        (
            match tok {
                Ok(tkn) => tkn,
                Err(_) => Token::Unknown(&src[span.clone()]),
            },
            span.into(),
        )
    });

    let eoi = SimpleSpan::from(src.len()..src.len());
    TokenStream::from_iter(lex_iter).spanned(eoi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_parser_test() {
        let src = "String?";
        let ast = type_parser().parse(wrap_lexer(src)).unwrap();
        let test = Type {
            name: "String".to_string(),
            is_array: false,
            is_required: false,
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn struct_parser_test() {
        let src = r"
        /// Struct comment
        struct Person: Parent {
            -- some comment
            /// Field [comment](https://vk.com)
            ///
            /// More **comments**
            field: String?,
            /// Field2 comment
            field2: String
        }";
        let ast = struct_parser().parse(wrap_lexer(src)).unwrap();
        let test = Struct {
            comment: Some("<p>Struct comment</p>\n".to_string()),
            name: "Person".to_string(),
            parent: Some("Parent".to_string()),
            fields: vec![
                Field {
                    name: "field".to_string(),
                    comment: "<p>Field <a href=\"https://vk.com\">comment</a></p>\n<p>More <strong>comments</strong></p>\n"
                        .to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    },
                },
                Field {
                    name: "field2".to_string(),
                    comment: "<p>Field2 comment</p>\n".to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: true,
                    },
                },
            ],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn enum_parser_test() {
        let src = r"
        /// Enum comment
        enum Color {
            /// Variant comment
            Red(String),
            /// Another comment
            Green,
            /// Blue comment
            Blue(String?),
        }";
        let ast = enum_parser().parse(wrap_lexer(src)).unwrap();
        let test = Enum {
            comment: Some("<p>Enum comment</p>\n".to_string()),
            name: "Color".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Red".to_string(),
                    comment: "<p>Variant comment</p>\n".to_string(),
                    field: Some(Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: true,
                    }),
                },
                EnumVariant {
                    name: "Green".to_string(),
                    comment: "<p>Another comment</p>\n".to_string(),
                    field: None,
                },
                EnumVariant {
                    name: "Blue".to_string(),
                    comment: "<p>Blue comment</p>\n".to_string(),
                    field: Some(Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    }),
                },
            ],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn module_parser_test() {
        let src = r"
        /// Struct comment
        struct Person: Parent {
            -- some comment
            /// Field [comment](https://vk.com)
            ///
            /// More **comments**
            field: String?,
            /// Field2 comment
            field2: String,
        }

        /// Enum comment
        enum Color {
            /// Variant comment
            Red(String),
            /// Another comment
            Green,
            /// Blue comment
            Blue(String?),
        }

        /// Variable comment
        let var: Person;
        ";

        let ast = module_parser().parse(wrap_lexer(src)).unwrap();

        let test = Module {
            type_defs: vec![
                TypeDef::Struct(Struct {
                    comment: Some("<p>Struct comment</p>\n".to_string()),
                    name: "Person".to_string(),
                    parent: Some("Parent".to_string()),
                    fields: vec![
                        Field {
                            name: "field".to_string(),
                            comment: "<p>Field <a href=\"https://vk.com\">comment</a></p>\n<p>More <strong>comments</strong></p>\n".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: false,
                            },
                        },
                        Field {
                            name: "field2".to_string(),
                            comment: "<p>Field2 comment</p>\n".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: true,
                            },
                        },
                    ],
                }),
                TypeDef::Enum(Enum {
                    comment: Some("<p>Enum comment</p>\n".to_string()),
                    name: "Color".to_string(),
                    variants: vec![
                        EnumVariant {
                            name: "Red".to_string(),
                            comment: "<p>Variant comment</p>\n".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: true,
                            }),
                        },
                        EnumVariant {
                            name: "Green".to_string(),
                            comment: "<p>Another comment</p>\n".to_string(),
                            field: None,
                        },
                        EnumVariant {
                            name: "Blue".to_string(),
                            comment: "<p>Blue comment</p>\n".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: false,
                            }),
                        },
                    ],
                }),
            ],
            variables: vec![Variable {
                name: "var".to_string(),
                comment: "<p>Variable comment</p>\n".to_string(),
                ty: Type {
                    name: "Person".to_string(),
                    is_array: false,
                    is_required: true,
                },
            }],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn array_parse_test() {
        let src = r"
        /// Person
        struct Person {
            /// Person kind
            kind: PersonKind,
            /// Field with array
            field: [String],
        }
        /// Person kind
        enum PersonKind {
            /// Newbie
            Newbie(NewbieInfo),
            /// Lawyer with a names
            Lawyer([String]),
        }
        /// Newbie info
        struct NewbieInfo {
            /// Newbie names
            field: [String],
        }
        /// Variable comment
        let var: [Person];
        ";

        let ast = module_parser().parse(wrap_lexer(src)).unwrap();

        let test = Module {
            type_defs: vec![
                TypeDef::Struct(Struct {
                    name: "Person".to_string(),
                    comment: Some("<p>Person</p>\n".to_string()),
                    fields: vec![
                        Field {
                            name: "kind".to_string(),
                            comment: "<p>Person kind</p>\n".to_string(),
                            ty: Type {
                                name: "PersonKind".to_string(),
                                is_array: false,
                                is_required: true,
                            },
                        },
                        Field {
                            name: "field".to_string(),
                            comment: "<p>Field with array</p>\n".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: true,
                            },
                        },
                    ],
                    parent: None,
                }),
                TypeDef::Enum(Enum {
                    comment: Some("<p>Person kind</p>\n".to_string()),
                    name: "PersonKind".to_string(),
                    variants: vec![
                        EnumVariant {
                            name: "Newbie".to_string(),
                            comment: "<p>Newbie</p>\n".to_string(),
                            field: Some(Type {
                                name: "NewbieInfo".to_string(),
                                is_array: false,
                                is_required: true,
                            }),
                        },
                        EnumVariant {
                            name: "Lawyer".to_string(),
                            comment: "<p>Lawyer with a names</p>\n".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: true,
                            }),
                        },
                    ],
                }),
                TypeDef::Struct(Struct {
                    comment: Some("<p>Newbie info</p>\n".to_string()),
                    name: "NewbieInfo".to_string(),
                    parent: None,
                    fields: vec![Field {
                        name: "field".to_string(),
                        comment: "<p>Newbie names</p>\n".to_string(),
                        ty: Type {
                            name: "String".to_string(),
                            is_array: true,
                            is_required: true,
                        },
                    }],
                }),
            ],
            variables: vec![Variable {
                name: "var".to_string(),
                comment: "<p>Variable comment</p>\n".to_string(),
                ty: Type {
                    name: "Person".to_string(),
                    is_array: true,
                    is_required: true,
                },
            }],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn empty_docs() {
        let source = "let var: String;";

        let res = parse_module(source);
        assert!(res.is_err());
    }
}
