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

use std::path::Path;

use self::cicero::compile_types;
use self::template::compile_template;
use super::scenario::Scenario;
use crate::types::ScenarioMeta;

pub mod cicero;
pub mod template;

fn parse_markdown(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn parse_meta(input: &str) -> Result<ScenarioMeta, String> {
    toml::from_str(input).map_err(|e| e.to_string())
}

pub fn compile_scenario(dir: impl AsRef<Path>) -> Result<Scenario, String> {
    let path = dir.as_ref();
    if !path.join("reference.docx").exists() {
        return Err("reference.docx doesn't exist".to_string());
    }
    let meta = std::fs::read_to_string(path.join("meta.toml")).map_err(|e| e.to_string())?;
    let types = std::fs::read_to_string(path.join("types.cicero")).map_err(|e| e.to_string())?;
    let template =
        std::fs::read_to_string(path.join("template.tex.j2")).map_err(|e| e.to_string())?;

    let meta = parse_meta(&meta)?;
    let var_env = compile_types(&types)?;
    let template = compile_template(&template, &var_env)?;

    Ok(Scenario::new(meta, template).expect("Already checked validity."))
}
