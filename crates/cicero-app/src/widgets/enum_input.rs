use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::{EntityInput, HtmlRender};

#[component]
pub fn EnumInput(
    enumeration: types::Enum,
    is_required: bool,
    data: RwSignal<data::Enum>,
) -> impl IntoView {
    todo!()
}
