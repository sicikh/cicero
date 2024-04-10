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

use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::HtmlRender;

// <section class="flex flex-col w-full h-full scrollbar-thumb-[#8C7456] scrollbar-thumb-h-[40px] scrollbar-track-[#eeeeee] hover:scrollbar-thumb-[#9c805d] active:scrollbar-thumb-[#9c805d]/50 scrollbar-h-[90%] scrollbar-w-[40px] overflow-y-scroll">
//     <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
//         <div class="flex flex-col gap-[10px] mb-[20px]">
//             <div class="font-bold">
//                 Введите данные покупателя:
//             </div>
//             <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
//                 Имя:
//                 <input
//                     class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
//                     type="text"
//                     placeholder="Имя"
//                     required
//                 />
//             </div>
//             <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
//                 Возраст:
//                 <input
//                     class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
//                     type="text"
//                     placeholder="Возраст"
//                     required
//                 />
//             </div>
//         </div>
//     </section>
//     <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
//         <div class="flex flex-row mb-[20px]">
//             <div class="font-bold pt-[10px]">
//                 Введите массив данных:
//             </div>
//             <div class="pl-[10px] flex flex-row p-[5px]">
//                 <button class="bg-[#eeeeee] w-[40px] h-[40px] border-[3px] border-[#8c7456] rounded-[50%]">
//                     <i class="bx bx-plus text-[#8c7456] pl-[1px] pt-[2px] text-[30px]"></i>
//                 </button>
//             </div>
//         </div>
//     </section>
//     <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
//         <div class="flex flex-col gap-[10px] mb-[20px]">
//             <div class="font-bold">
//                 Введите адрес покупателя:
//             </div>
//             <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
//                 <input
//                     class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
//                     type="text"
//                     placeholder="Адрес"
//                     required
//                 />
//             </div>
//         </div>
//     </section>
// </section>

#[component]
pub fn EntityInput(
    entity: types::Entity,
    placeholder: String,
    data: RwSignal<data::Data>
) -> impl IntoView {
    let is_required = entity.is_required;
    data.with(|data| match (entity.ty, data) {
        (types::EntityType::Struct(structure), data::Data::Struct(data)) => {
            view! { <StructInput structure is_required data=*data/> }
        },
        (types::EntityType::String, data::Data::String(data)) => {
            view! { <StringInput placeholder is_required data=*data/> }
        },
        _ => {
            unreachable!("Data/type mismatch in EntityInput")
        }
    })
}

#[component]
pub fn StructInput(
    structure: types::Struct,
    is_required: bool,
    data: RwSignal<data::Struct>,
) -> impl IntoView {
    view! {
        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
            <div class="flex flex-col gap-[10px] mb-[20px]">

                {
                    let header = structure
                        .comment
                        .map(|comment| {
                            view! {
                                <div class="font-bold">
                                    <HtmlRender html_string=comment/>
                                </div>
                            }
                        });
                    let fields = structure
                        .fields
                        .into_values()
                        .zip(data().fields.into_values())
                        .map(|(type_field, data_field)| {
                            view! {
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <HtmlRender html_string=type_field.comment.clone()/>
                                    <EntityInput
                                        entity=type_field.entity
                                        placeholder=type_field.comment
                                        data=data_field
                                    />
                                </div>
                            }
                        })
                        .collect_view();
                    (header, fields)
                }

            </div>
        </section>
    }
}

#[component]
pub fn StringInput(
    placeholder: String,
    is_required: bool,
    data: RwSignal<String>
) -> impl IntoView {
    view! {
        <input
            class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
            type="text"
            placeholder=placeholder
            required=is_required
            prop:value=data
            on:input=move |ev| data.set(event_target_value(&ev))
        />
    }
}
