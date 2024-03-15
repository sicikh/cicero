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

// FIXME
#![allow(dead_code, unused_imports, unused_variables, clippy::manual_try_fold)]

#[cfg(feature = "render")]
pub mod compiler;
pub mod data;
pub mod std_lib;
pub mod types;
