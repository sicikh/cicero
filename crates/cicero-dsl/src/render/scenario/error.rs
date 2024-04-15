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

pub type Result<T> = core::result::Result<T, ScenarioError>;

#[derive(thiserror::Error, Debug)]
pub enum ScenarioError {
    #[error("Step {0} is out of bounds.")]
    StepOutOfBounds(usize),
    #[error("Step {0} data is not filled.")]
    StepNotFilled(usize),
    #[error("Step {0} data is not valid.")]
    StepNotValid(usize),
    #[error(transparent)]
    TemplateError(#[from] minijinja::Error),
    #[error("Failed to write a file.")]
    FileWriteError(std::io::Error),
    #[error("Tectonic failed to compile the LaTeX file.")]
    TectonicError(std::io::Error),
    #[error("Pandoc failed to convert the LaTeX file to DOCX.")]
    PandocError(std::io::Error),
}
