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
pub fn Contacts() -> impl IntoView {
    view! {
        <Layout>
            <div class="flex flex-col h-full">
                <section class="flex flex-col flex-1 items-center pt-[40px] bg-[#eeeeee]">
                    <div class="text-center px-3 lg:px-0">
                        <h1 class="my-4 text-[74px] px-[150px] text-[#8C7456] leading-tight w-[1/2]">
                            Контакты
                        </h1>
                    </div>
                </section>
            </div>
        </Layout>
    }
}
