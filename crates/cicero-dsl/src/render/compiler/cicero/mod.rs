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

pub use resolver::check_data_validity;

use self::grammar::parse_module;
use self::resolver::resolve;
use crate::render::context::VarEnv;

mod ast;
mod grammar;
mod lexer;
mod resolver;

pub fn compile_types(source: &str) -> Result<VarEnv, String> {
    let module = parse_module(source)?;
    let var_env = resolve(module)?;
    Ok(var_env)
}
