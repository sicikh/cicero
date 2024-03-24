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

use crate::render::context::VarEnv;
use crate::render::scenario::Template;

mod ast;
mod grammar;
mod resolver;

pub fn compile_template(template: &str, var_env: &VarEnv) -> Result<Template, String> {
    let ast = grammar::parse_template(template)?;

    let template = resolver::resolve_template(ast, var_env)?;

    Ok(template)
}
