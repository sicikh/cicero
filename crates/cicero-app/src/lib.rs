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
            <main class="h-screen">
                <Routes>
                    <Route path="/Konstruktor" view=Konstruktor/>
                    <Route path="/" view=Maga/>
                    <Route path="/Contact" view=Contact/>
                    <Route path="/Login" view=Login/>
                    <Route path="/Register" view=Register/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn Maga() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <header class="bg-[#8C7456] h-[180px]">
            <nav
                id="nav-bar"
                class="md:flex md:justify-between md:items-center w-[92%] mx-auto h-[180px]"
            >
                <div id="logo" class="md:flex items-start text-center">
                    <p class="text-white text-[40px] mt-[60px] mb-[60px]">Cicero</p>
                </div>
                <div class="text-inter text-center pl-36 font-light" id="nav-bar-container-elem">
                    <ul class="md:flex md:items-center gap-[22px] md:justify-center">
                        <li
                            id="nav-bar-main"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px] "
                        >
                            <a href="/" style="text-center">
                                Главная
                            </a>
                        </li>
                        <li
                            id="nav-bar-kit"
                            class="text-white text-center items-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Konstruktor" style="text-center">
                                Конструктор
                            </a>
                        </li>
                        <li
                            id="nav-bar-contact"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Contact" style="text-center">
                                Контакты
                            </a>
                        </li>
                        <div class="text-center gap-y-[15px] grid items-end">
                            <div class="text-center" id="button_entry">
                                <a href="Login">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Войти
                                    </button>
                                </a>
                            </div>
                            <div class="text-center" id="button_entry1">
                                <a href="Register">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Зарегистрироваться
                                    </button>
                                </a>
                            </div>
                        </div>
                    </ul>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Konstruktor() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <header class="bg-[#8C7456] h-[180px]">
            <nav
                id="nav-bar"
                class="md:flex md:justify-between md:items-center w-[92%] mx-auto h-[180px]"
            >
                <div id="logo" class="md:flex items-start text-center">
                    <p class="text-white text-[40px] mt-[60px] mb-[60px]">Cicero</p>
                </div>
                <div class="text-inter text-center pl-36 font-light" id="nav-bar-container-elem">
                    <ul class="md:flex md:items-center gap-[22px] md:justify-center">
                        <li
                            id="nav-bar-main"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px] "
                        >
                            <a href="/" style="text-center">
                                Главная
                            </a>
                        </li>
                        <li
                            id="nav-bar-kit"
                            class="text-white text-center items-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Konstruktor" style="text-center">
                                Конструктор
                            </a>
                        </li>
                        <li
                            id="nav-bar-contact"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Contact" style="text-center">
                                Контакты
                            </a>
                        </li>
                        <div class="text-center gap-y-[15px] grid items-end">
                            <div class="text-center" id="button_entry">
                                <a href="Login">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Войти
                                    </button>
                                </a>
                            </div>
                            <div class="text-center" id="button_entry1">
                                <a href="Register">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Зарегистрироваться
                                    </button>
                                </a>
                            </div>
                        </div>
                    </ul>
                </div>
            </nav>
        </header>
        <div id="main_body">
            <div id="left_side" class="md:flex md:items-center w-1/2">
                <section id="search" class="w-full h-[73px] bg-[#EEEEEE]">
                    <div class="justify-between h-[37px] mt-[18px] mb-[18px] ml-[25px] mr-[25px] relative">
                        <input
                            type="search"
                            class="w-full h-full outline-none bg-[#261201] bg-opacity-[81%] border-solid border-[3px] rounded-[10px] border-[#8C7456] placeholder-[#A1A1A1] text-[16px] font-light pl"
                            name="search-text"
                            placeholder="Поиск документов"
                        />
                        <i class="bx bx-search text-[#8C7456] text-[25px] absolute"></i>
                    </div>
                </section>
                <section id="find_doc"></section>
            </div>
            <div id="balka_ebanay" class="w-[14px] h-full bg-[#8C7456]"></div>
            <div id="right_side"></div>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <header class="bg-[#8C7456] h-[180px]">
            <nav
                id="nav-bar"
                class="md:flex md:justify-between md:items-center w-[92%] mx-auto h-[180px]"
            >
                <div id="logo" class="md:flex items-start text-center">
                    <p class="text-white text-[40px] mt-[60px] mb-[60px]">Cicero</p>
                </div>
                <div class="text-inter text-center pl-36 font-light" id="nav-bar-container-elem">
                    <ul class="md:flex md:items-center gap-[22px] md:justify-center">
                        <li
                            id="nav-bar-main"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px] "
                        >
                            <a href="/" style="text-center">
                                Главная
                            </a>
                        </li>
                        <li
                            id="nav-bar-kit"
                            class="text-white text-center items-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Konstruktor" style="text-center">
                                Конструктор
                            </a>
                        </li>
                        <li
                            id="nav-bar-contact"
                            class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                        >
                            <a href="Contact" style="text-center">
                                Контакты
                            </a>
                        </li>
                        <div class="text-center gap-y-[15px] grid items-end">
                            <div class="text-center" id="button_entry">
                                <a href="Login">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Войти
                                    </button>
                                </a>
                            </div>
                            <div class="text-center" id="button_entry1">
                                <a href="Register">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Зарегистрироваться
                                    </button>
                                </a>
                            </div>
                        </div>
                    </ul>
                </div>
            </nav>
        </header>
        <div>
            <h1>13</h1>
            <h1>43</h1>
            <h1>43</h1>
        </div>
    }
}

#[component]
fn Login() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div
            id="wrapper"
            class="md:flex md:justify-center md:items-center min-h-screen bg-[#8C7456]"
        >
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
                <div
                    id="remember-forgot"
                    class="md:flex justify-between text-[15px] mt-[15px] mb-[15px] mr-0 ml-0 "
                >
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
/// Test and all of that
fn Register() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <div
            id="wrapper"
            class="md:flex md:justify-center md:items-center min-h-screen bg-[#8C7456]"
        >

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
                <div
                    id="remember-forgot"
                    class="md:flex justify-end text-[15px] mt-[15px] mb-[15px] mr-0 ml-0 "
                >
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
                        Есть аккаунт?
                        <a href="Login" class="font-semibold hover:underline ml-[5px]">
                            Войти
                        </a>
                    </p>
                </div>
            </form>
        </div>
    }
}
