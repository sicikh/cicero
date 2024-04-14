use cicero_dsl::types;
use cicero_dsl::types::HtmlString;
use leptos::*;

use crate::data::data_from_entity;
use crate::shared::data;
use crate::widgets::{EntityInput, HtmlEnumRender, HtmlRender};

#[component]
pub fn StringEnumInput(html_string: HtmlString, id: String, name: String) -> impl IntoView {
    view! {
        <input class="mr-[5px]" type="radio" id=id.clone() name=name.clone()/>
        <HtmlEnumRender html_string=html_string name=id.clone()/>
    }
}
