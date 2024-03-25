/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>,
 * Gleb Krylov <gleb_cry@mail.ru>
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
use crate::types::MarkdownString;

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

/// struct ::= [ comment ] 'struct' ident [ ':' ident ] '{' fields methods '}'
fn struct_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Struct, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    comment_parser()
        .or_not()
        .then_ignore(just(Token::STRUCT))
        .then(ident_parser().then(just(Token::Colon).ignore_then(ident_parser()).or_not()))
        .then(
            fields_parser()
                .then(methods_parser())
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map(|((comment, (name, parent)), (fields, methods))| {
            Struct {
                comment,
                name,
                parent,
                fields,
                methods,
            }
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
) -> impl Parser<'a, I, MarkdownString, extra::Err<Rich<'a, Token<'a>>>> + Copy {
    let comment = select! {
        Token::DocComment(doc) => doc.replace("/// ", "").replace("///", ""),
    };

    comment
        .labelled("comment")
        .repeated()
        .collect::<Vec<_>>()
        .map(|comments| comments.join("").trim().to_string())
}

/// methods ::= ( method )*
fn methods_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Vec<Method>, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    method_parser().repeated().collect()
}

/// 'fn' ident '{ expr '}'
fn method_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Method, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    just(Token::FN)
        .ignore_then(ident_parser())
        .then(expr_parser().delimited_by(just(Token::LBrace), just(Token::RBrace)))
        .map(|(name, body)| Method { name, body })
}

// TODO
/// expr ::=
fn expr_parser<'a, I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>>(
) -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>> + Clone {
    just(Token::Asterisk).map(|_| Expr::Todo)
}

/// enum ::= comment 'enum' ident '{' enum_variant [ ',' enum_variant ]* [ ',' ]
/// methods '}'
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
                .then(methods_parser())
                .delimited_by(just(Token::LBrace), just(Token::RBrace)),
        )
        .map(|((comment, name), (variants, methods))| {
            Enum {
                name,
                comment,
                variants,
                methods,
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
        let src = r#"
        /// Struct comment
        struct Person: Parent {
            -- some comment
            /// Field comment
            ///
            /// More **comments**
            field: String?,
            /// Field2 comment
            field2: String
        }"#;
        let ast = struct_parser().parse(wrap_lexer(src)).unwrap();
        let test = Struct {
            comment: Some("Struct comment".to_string()),
            name: "Person".to_string(),
            parent: Some("Parent".to_string()),
            fields: vec![
                Field {
                    name: "field".to_string(),
                    comment: "Field comment\n\nMore **comments**".to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    },
                },
                Field {
                    name: "field2".to_string(),
                    comment: "Field2 comment".to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: true,
                    },
                },
            ],
            methods: vec![],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn enum_parser_test() {
        let src = r#"
        /// Enum comment
        enum Color {
            /// Variant comment
            Red(String),
            /// Another comment
            Green,
            /// Blue comment
            Blue(String?),
        }"#;
        let ast = enum_parser().parse(wrap_lexer(src)).unwrap();
        let test = Enum {
            comment: Some("Enum comment".to_string()),
            name: "Color".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Red".to_string(),
                    comment: "Variant comment".to_string(),
                    field: Some(Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: true,
                    }),
                },
                EnumVariant {
                    name: "Green".to_string(),
                    comment: "Another comment".to_string(),
                    field: None,
                },
                EnumVariant {
                    name: "Blue".to_string(),
                    comment: "Blue comment".to_string(),
                    field: Some(Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    }),
                },
            ],
            methods: vec![],
        };

        assert_eq!(ast, test);
    }

    #[test]
    fn module_parser_test() {
        let src = r#"
        /// Struct comment
        struct Person: Parent {
            -- some comment
            /// Field comment
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
        "#;

        let ast = module_parser().parse(wrap_lexer(src)).unwrap();

        let test = Module {
            type_defs: vec![
                TypeDef::Struct(Struct {
                    comment: Some("Struct comment".to_string()),
                    name: "Person".to_string(),
                    parent: Some("Parent".to_string()),
                    fields: vec![
                        Field {
                            name: "field".to_string(),
                            comment: "Field comment\n\nMore **comments**".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: false,
                            },
                        },
                        Field {
                            name: "field2".to_string(),
                            comment: "Field2 comment".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: true,
                            },
                        },
                    ],
                    methods: vec![],
                }),
                TypeDef::Enum(Enum {
                    comment: Some("Enum comment".to_string()),
                    name: "Color".to_string(),
                    variants: vec![
                        EnumVariant {
                            name: "Red".to_string(),
                            comment: "Variant comment".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: true,
                            }),
                        },
                        EnumVariant {
                            name: "Green".to_string(),
                            comment: "Another comment".to_string(),
                            field: None,
                        },
                        EnumVariant {
                            name: "Blue".to_string(),
                            comment: "Blue comment".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: false,
                                is_required: false,
                            }),
                        },
                    ],
                    methods: vec![],
                }),
            ],
            variables: vec![Variable {
                name: "var".to_string(),
                comment: "Variable comment".to_string(),
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
        let src = r#"
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
        "#;

        let ast = module_parser().parse(wrap_lexer(src)).unwrap();

        let test = Module {
            type_defs: vec![
                TypeDef::Struct(Struct {
                    name: "Person".to_string(),
                    comment: Some("Person".to_string()),
                    fields: vec![
                        Field {
                            name: "kind".to_string(),
                            comment: "Person kind".to_string(),
                            ty: Type {
                                name: "PersonKind".to_string(),
                                is_array: false,
                                is_required: true,
                            },
                        },
                        Field {
                            name: "field".to_string(),
                            comment: "Field with array".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: true,
                            },
                        },
                    ],
                    parent: None,
                    methods: vec![],
                }),
                TypeDef::Enum(Enum {
                    comment: Some("Person kind".to_string()),
                    name: "PersonKind".to_string(),
                    variants: vec![
                        EnumVariant {
                            name: "Newbie".to_string(),
                            comment: "Newbie".to_string(),
                            field: Some(Type {
                                name: "NewbieInfo".to_string(),
                                is_array: false,
                                is_required: true,
                            }),
                        },
                        EnumVariant {
                            name: "Lawyer".to_string(),
                            comment: "Lawyer with a names".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: true,
                            }),
                        },
                    ],
                    methods: vec![],
                }),
                TypeDef::Struct(Struct {
                    comment: Some("Newbie info".to_string()),
                    name: "NewbieInfo".to_string(),
                    parent: None,
                    fields: vec![Field {
                        name: "field".to_string(),
                        comment: "Newbie names".to_string(),
                        ty: Type {
                            name: "String".to_string(),
                            is_array: true,
                            is_required: true,
                        },
                    }],
                    methods: vec![],
                }),
            ],
            variables: vec![Variable {
                name: "var".to_string(),
                comment: "Variable comment".to_string(),
                ty: Type {
                    name: "Person".to_string(),
                    is_array: true,
                    is_required: true,
                },
            }],
        };

        assert_eq!(ast, test);
    }
}
