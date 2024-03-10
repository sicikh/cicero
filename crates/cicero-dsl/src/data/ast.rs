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

use serde::{Deserialize, Serialize};

use super::Data;

/// Syntax tree for the methods on types.
#[derive(Serialize, Deserialize, Debug)]
pub enum Method {}

pub fn eval(data: &Data, ast: &Method) -> Option<String> {
    unimplemented!()
}
