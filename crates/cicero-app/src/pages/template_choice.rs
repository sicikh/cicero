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

use std::collections::HashMap;

use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;

use crate::widgets::*;

#[component]
pub fn TemplateChoice() -> impl IntoView {
    let metas = vec![ScenarioMeta {
        id: 0,
        name: "Test".to_string(),
        description: "Test".to_string(),
        category: "Testing".to_string(),
    }];

    let categories: HashMap<String, Vec<ScenarioMeta>> =
        metas.into_iter().fold(HashMap::new(), |mut map, meta| {
            map.entry(meta.category.clone())
                .and_modify(|entry| entry.push(meta.clone()))
                .or_insert(vec![meta]);
            map
        });
    //let (scenario, dildo) = create_signal(vec![new(){
    //    25;
    //   "dog".to_string();
    //}]);
    // Creates a reactive value to update the button
    view! {
        <LayoutNav>

            <div id="main_body" class="md: flex flex-row h-full bg-[#EEEEEE]">
                <div
                    id="left_side"
                    class="md:flex flex-col border-r-[7px] border-[#8C7456] w-1/2 basis-1/2"
                >
                    <section id="search" class="w-full h-[73px] relative bg-[#EEEEEE]">
                        <div class="justify-between items-center h-[37px] mt-[18px] mb-[18px] ml-[25px] mr-[25px] relative">
                            <input
                                type="search"
                                class="w-full pl-[30px] h-full absolute outline-none bg-[#261201] bg-opacity-[81%] border-solid border-[3px] rounded-[10px] border-[#8C7456] placeholder-[#A1A1A1] text-[#A1A1A1] text-[16px] font-light pl"
                                name="search-text"
                                placeholder="Поиск документов"
                            />
                            <i class="bx bx-search items-center pt-[8px] pl-[7px] text-[#8C7456] text-[25px] absolute"></i>
                        </div>
                    </section>
                    <section
                        id="choice"
                        class="flex flex-row justify-start w-full h-[40px] relative bg-[#EEEEEE]"
                    >
                        <button class="rounded-tr-[10px] border-t-[3px] border-r-[3px] border-b-[3px] w-[115px] border-[#8C7456] items-center text-center">
                            <div class="text-[#8C7456] hover:text-[#BFA07A]">
                                Документы
                            </div>
                        </button>
                        <button class="rounded-t-[10px] border-[3px] border-[#8C7456] w-[160px] items-center text-center">
                            <div class="text-[#8C7456] hover:text-[#BFA07A]">
                                Учетная политика
                            </div>
                        </button>
                        <button class="rounded-t-[10px] border-[3px] border-[#8C7456] w-[130px] items-center text-center">
                            <div class="text-[#8C7456] hover:text-[#BFA07A]">
                                Мои проекты
                            </div>
                        </button>
                        <button class="rounded-tl-[10px] flex-1 border-t-[3px] border-l-[3px] border-b-[3px] border-[#8C7456] items-center text-center">
                            <div class="text-[#8C7456] hover:text-[#BFA07A]"></div>
                        </button>
                    </section>
                    <LayoutChoiceDogovor>
                        <ChoiceAElectionDogovor/>
                        <ChoiceADogovor/>
                    </LayoutChoiceDogovor>
                </div>
                // посмотрим     // <div id="balka_ebanay" class="md:flex w-[14px] h-full bg-[#8C7456]"></div>
                <RightSide/>
            </div>
        </LayoutNav>
    }
}
