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
            <div class="h-full bg-[#BFA07A]">
                <div class="text-[#eeeeee]">
                    <p>
                        На нашем сайте-конструкторе документов вы получаете удобный инструмент для создания юридически значимых документов. Просто введите необходимую информацию, следуя интуитивно понятным шагам, и наш сервис сгенерирует для вас профессионально оформленный PDF-документ.

                        Это быстрый и надежный способ получить готовый договор или другой юридический документ, соответствующий вашим потребностям. Сэкономьте время и упростите процесс создания документации с нашим конструктором документов.

                    </p>
                </div>
                <div>
                    <a href="/scenario" style="text-center">
                        <button class="text-[#eeeeee]">Попробовать</button>
                    </a>

                </div>
            </div>
        </Layout>
    }
}
