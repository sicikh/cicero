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

mod ast;
mod grammar;
mod lexer;
mod resolver;

#[cfg(test)]
mod tests {
    use super::grammar::parse_module;
    use super::resolver::resolve;
    // FIXME: comments is empty, but it parses
    #[test]
    fn basic_test() {
        let src = r#"
        struct Person { name: String }
        struct PersonWithSurname: Person { surname: String }
        struct PersonWithDate: Person { date: String }
        enum Various {
            Person(Person),
            PersonWithSurname(PersonWithSurname),
            PersonWithDate(PersonWithDate),
            Nothing,
        }

        let p: Various
        "#;
        let module = parse_module(src).unwrap();
        let module = resolve(module).unwrap();
        // TODO: write test, but it works for now...
        println!("{:#?}", module);
    }
}
