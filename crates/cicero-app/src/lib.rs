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
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
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
                <Route path="/contacts" view=Contacts/>
                <Route path="/login" view=Login/>
                <Route path="/register" view=Register/>
            </Routes>
        </Router>
    }
}

#[component]
fn Login() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div id="wrapper" class="md:flex md:justify-center md:items-center min-h-screen bg-[#8C7456]">
            <form
                action=""
                class="w-[420px] bg-transparent border-solid border-2 border-[#ffffff33] backdrop-blur-[20px] text-[#EEEEEE] rounded-[10px] pt-[30px] pb-[30px] pl-[40px] pr-[40px]"
            >
                <h1 class="text-center text-[36px] font-bold">Войти</h1>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px]"
                        type="text"
                        placeholder="Логин"
                        required
                    />
                    <i class="bx bx-user absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px] "
                        type="password"
                        placeholder="Пароль"
                        required
                    />
                    <i class="bx bxs-lock-alt absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div id="remember-forgot" class="md:flex justify-between text-[15px] mt-[15px] mb-[15px] mr-0 ml-0 ">
                    <label class="accent-[#EEEEEE]">
                        <input type="checkbox" class="mr-[3px] ml-[1px]"/>
                        Запомнить пароль
                    </label>
                    <a href="#" class="hover:underline">
                        Забыли пароль?
                    </a>
                </div>
                <button
                    id="but_login"
                    class="w-full h-[45px] bg-[#EEEEEE] border-none rounded-[40px] outline-none shadow-[0_0_10px_rgba(0,0,0,0.1) text-[16px] text-[#333] font-bold"
                    type="submit"
                >
                    Login
                </button>
                <div id="Register_link" class="text-[15px] text-center mt-[15px]">
                    <p>
                        <a href="Register" class="font-semibold hover:underline">
                            Создать аккаунт
                        </a>
                    </p>
                </div>
            </form>
        </div>
    }
}

#[component]
fn Register() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div id="wrapper" class="md:flex md:justify-center md:items-center min-h-screen bg-[#8C7456]">

            <form
                action=""
                class="w-[420px] bg-transparent border-solid border-2 border-[#ffffff33] backdrop-blur-[20px] text-[#EEEEEE] rounded-[10px] pt-[30px] pb-[30px] pl-[40px] pr-[40px]"
            >
                <h1 class="text-center text-[36px] font-bold">Регистрация</h1>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px]"
                        type="text"
                        placeholder="Имя"
                        required
                    />
                    <i class="bx bx-user absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px]"
                        type="text"
                        placeholder="Фамилия"
                        required
                    />
                    <i class="bx bx-user absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px]"
                        type="text"
                        placeholder="Логин"
                        required
                    />
                    <i class="bx bx-user absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px]"
                        type="email"
                        placeholder="E-mail"
                        required
                    />
                    <i class="bx bx-user absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div class="w-full h-[50px] mt-[30px] mb-0 relative">
                    <input
                        id="input-box"
                        class="w-full h-full bg-transparent outline-none border-solid border-2 border-[#ffffff33] rounded-[40px] placeholder-[#EEEEEE] text-[18px] pb-[20px] pr-[45px] pt-[20px] pl-[20px] "
                        type="password"
                        placeholder="Пароль"
                        required
                    />
                    <i class="bx bxs-lock-alt absolute right-[20px] text-[20px] top-[50%] -translate-y-1/2"></i>
                </div>
                <div id="remember-forgot" class="md:flex justify-end text-[15px] mt-[15px] mb-[15px] mr-0 ml-0 ">
                    <a href="#" class="hover:underline">
                        Забыли пароль?
                    </a>
                </div>
                <button
                    id="but_login"
                    class="w-full h-[45px] bg-[#EEEEEE] border-none rounded-[40px] outline-none shadow-[0_0_10px_rgba(0,0,0,0.1) text-[16px] text-[#333] font-bold"
                    type="submit"
                >
                    Login
                </button>
                <div id="Register_link" class="text-[15px] text-center mt-[15px]">
                    <p>
                        Есть аккаунт? <a href="Login" class="font-semibold hover:underline ml-[5px]">
                            Войти
                        </a>
                    </p>
                </div>
            </form>
        </div>
    }
}
