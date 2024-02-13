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

use error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/cicero.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@100;200;300;400;500;600;700;800;900&display=swap" rel="stylesheet"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="h-screen">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <header class="bg-[#8C7456] h-[180px]">
            <nav id="nav-bar" class="md:flex md:justify-between md:items-center w-[92%] mx-auto h-[180px]">
                <div id="logo" class = "md:flex items-start text-center">
                    <p class = "text-white text-[40px] mt-[60px] mb-[60px]">Cicero</p>
                </div>
                <div class="text-inter text-center pl-36 font-light" id="nav-bar-container-elem">
                    <ul class="md:flex md:items-center gap-[22px] md:justify-center">
                        <li id="nav-bar-main" class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px] ">
                            <a href="#" style = "text-center">Главная</a>
                        </li>
                        <li id="nav-bar-kit" class="text-white text-center items-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]">
                            <a href="#" style = "text-center">Конструктор</a>
                        </li>
                        <li id="nav-bar-contact" class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]">
                            <a href="#" style = "text-center">Контакты</a>
                        </li>
                        <div class = "text-center gap-y-[15px] grid items-end">   
                            <div class="text-center" id="button_entry">
                                <button id="nav-bar-but2" class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]">Войти</button>
                            </div>
                            <div class="text-center" id="button_entry1">
                                <button id="nav-bar-but2" class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]">Зарегистрироваться</button>
                            </div>
                        </div>
                    </ul>
                </div>
            </nav>
        </header>
    }
}
