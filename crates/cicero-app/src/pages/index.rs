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

use leptos::*;
use leptos_meta::*;

use crate::widgets::*;

#[component]
pub fn MainPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="flex flex-col h-full">
                <section class="flex flex-col flex-1 items-center pt-[40px] bg-[#eeeeee]">
                    <div class="text-center px-3 lg:px-0">
                        <h1 class="my-4 text-[36px] px-[50px] text-[#8C7456] leading-tight w-[1/2]">
                            Проект "Cicero"
                            - это удобный инструмент для создания юридически значимых документов
                        </h1>
                        <p class="leading-normal text-[30px] px-[300px] text-[#8C7456] text-base mb-8">
                            Это быстрый и надежный способ получить готовый договор или другой юридический документ, соответствующий вашим потребностям. Сэкономьте время и упростите процесс создания документации с нашим конструктором документов.
                        </p>
                        <a href="/scenario">
                            <button class="mx-auto rounded-[3px] lg:mx-0 hover:underline bg-[#ebe8e8] text-[#8c7456] font-extrabold my-2 md:my-6 py-4 px-8 shadow-2xl w-56">
                                Попробовать
                            </button>
                        </a>
                    </div>
                </section>
                <footer class="bg-[#8C7456]">
                    <div class="container max-w-6xl mx-auto flex items-center py-5">

                        <div class="w-full mx-[25px] flex flex-wrap items-center">
                            <div class="flex w-full md:w-1/2 justify-center md:justify-start text-white font-extrabold">
                                <a class="text-[#8C7456] no-underline hover:text-[#8C7456] hover:no-underline" href="#">
                                    <span class="text-base text-gray-200 text-[30px]">Cicero</span>
                                </a>
                            </div>
                            <div class="flex w-full pt-2 content-center justify-between md:w-1/2 md:justify-end">
                                <ul class="list-reset flex justify-center flex-1 md:flex-none items-center">
                                    <li>
                                        <a
                                            class="inline-block text-[#EEEEEE] no-underline hover:underline py-2 px-3"
                                            href="/"
                                        >
                                            Главная
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="inline-block text-[#EEEEEE] no-underline hover:underline py-2 px-3"
                                            href="/scenario"
                                        >
                                            Конструктор
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="inline-block text-[#EEEEEE] no-underline hover:underline py-2 px-3"
                                            href="/contacts"
                                        >
                                            Контакты
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>

                    </div>
                </footer>
            </div>
        </Layout>
    }
}
