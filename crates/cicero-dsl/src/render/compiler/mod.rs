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

pub mod cicero;
use std::path::Path;

pub use cicero::*;
pub mod template;
pub use template::*;

use super::context::{Context, VarEnv};
use super::scenario::Scenario;
use crate::types::ScenarioMeta;

fn parse_meta(input: &str) -> Result<ScenarioMeta, String> {
    todo!()
}

pub fn compile_scenario(dir: impl AsRef<Path>) -> Result<Scenario, String> {
    todo!()
}
