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

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub beginning_clause: String,
    pub steps: Vec<Step>,
    pub end_clause: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub name: String,
    pub comment: Option<String>,
    pub variables: Vec<String>,
    pub body: String,
}
