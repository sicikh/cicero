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

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::empty_docs,
    non_snake_case
)]

use cfg_if::cfg_if;
use error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod error_template;
mod pages;
use self::pages::*;
mod widgets;
use self::widgets::*;
mod shared;
use self::shared::*;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        pub use self::shared::server;
    }
);

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/cicero.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link
            href="https://fonts.googleapis.com/css2?family=Poppins:wght@100;200;300;400;500;600;700;800;900&display=swap"
            rel="stylesheet"
        />
        <link href="https://unpkg.com/boxicons@2.1.4/css/boxicons.min.css" rel="stylesheet"/>

        // TODO: add support for nested routing in the future
        // https://book.leptos.dev/router/17_nested_routing.html
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route path="/scenario" view=ScenarioChoice/>
                <Route path="/scenario/:id" view=ScenarioStep/>
                <Route path="/scenario/:id/:step" view=ScenarioStep/>
                <Route path="/" view=MainPage/>
            </Routes>
        </Router>
    }
}
