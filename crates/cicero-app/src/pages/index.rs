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
                        <h1 class="my-4 text-[40px] px-[150px] text-[#8C7456] leading-tight w-[1/2]">
                            Проект "Cicero"
                            - это удобный инструмент для создания юридически значимых документов
                        </h1>
                        <p class="leading-normal text-xl px-[400px] text-[#8C7456] mb-8">
                            Это быстрый и надежный способ получить готовый договор или другой юридический документ, соответствующий вашим потребностям. Сэкономьте время и упростите процесс создания документации с нашим конструктором документов.
                        </p>
                        <a href="/scenario">
                            <button class="mx-auto rounded-[6px] lg:mx-0 hover:underline bg-[#8c7456] text-[#ebe8e8] font-extrabold my-2 md:my-6 py-4 px-6 shadow-2xl w-80">
                                Создать свой первый документ
                            </button>
                        </a>
                    </div>
                </section>
            </div>
        </Layout>
    }
}
