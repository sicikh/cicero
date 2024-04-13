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
use crate::widgets::{EntityInput, HtmlRender};

#[component]
pub fn StructInput(
    structure: types::Struct,
    is_required: bool,
    data: RwSignal<data::Struct>,
    recursion_level: usize,
) -> impl IntoView {
    let height = 50;
    view! {
        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
            <div class="flex flex-col gap-[10px] mb-[20px]">

                {move || {
                    let header = structure
                        .comment
                        .as_ref()
                        .map(|comment| {
                            view! {
                                <div class="font-bold">
                                    <HtmlRender html_string=comment.clone()/>
                                </div>
                            }
                        });
                    let fields = structure
                        .fields
                        .clone()
                        .into_values()
                        .zip(data().fields.into_values())
                        .map(|(type_field, data_field)| {
                            view! {
                                // "pl-[20px] flex flex-row gap-x-[5px] items-center"
                                <div class=match recursion_level {
                                    0 => "pl-[20px] flex flex-row gap-x-[5px] items-center",
                                    1..=3 => "pl-[20px] flex flex-row gap-x-[5px] items-center",
                                    _ => panic!("Recursion level is too big: {}", recursion_level),
                                }>
                                    <HtmlRender html_string=type_field.comment.clone()/>
                                    <EntityInput
                                        entity=type_field.entity
                                        placeholder=type_field.comment
                                        data=data_field
                                        recursion_level=recursion_level + 1
                                    />
                                </div>
                            }
                        })
                        .collect_view();
                    (header, fields).into_view()
                }}

            </div>
        </section>
    }
}
