/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use indexmap::IndexMap;

use self::grammar::parse_module;
use self::resolver::resolve;
use crate::types;

pub type VarEnv = IndexMap<String, types::Var>;

mod ast;
mod grammar;
mod lexer;
mod resolver;

pub fn compile_types(source: &str) -> Result<VarEnv, String> {
    let module = parse_module(source)?;
    let var_env = resolve(module)?;
    Ok(var_env)
}

fn parse_markdown(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}