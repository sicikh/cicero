use cicero_dsl::types;
use cicero_dsl::types::HtmlString;
use leptos::*;

use crate::data::data_from_entity;
use crate::shared::data;
use crate::widgets::{EntityInput, HtmlEnumRender, HtmlRender};

#[component]
pub fn StringEnumInput(
    html_string: HtmlString,
    id: String,
    name: String,
    value: usize,
    selected: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <input
            class="mr-[5px]"
            type="radio"
            id=id.clone()
            name=name.clone()
            prop:value=value
            on:change=move |ev| {
                let input: web_sys::HtmlInputElement = event_target(&ev);
                let name = input.value().parse::<usize>().unwrap();
                selected.set(name);
            }
        />

        <HtmlEnumRender html_string=html_string name=id.clone()/>
    }
}
