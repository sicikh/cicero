use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::{EnumInput, StringInput, StructInput};

#[component]
pub fn EntityInput(
    entity: types::Entity,
    placeholder: String,
    data: RwSignal<data::Data>,
    recursion_level: usize,
) -> impl IntoView {
    let is_required = entity.is_required;
    // CHECK: closure?
    data.with(|data| {
        match (entity.ty, data) {
            (types::EntityType::Struct(structure), data::Data::Struct(data)) => {
                view! { <StructInput structure is_required data=*data recursion_level/> }
            },
            (types::EntityType::String, data::Data::String(data)) => {
                view! { <StringInput placeholder is_required data=*data recursion_level/> }
            },
            (types::EntityType::Enum(enumeration), data::Data::Enum(data)) => {
                view! { <EnumInput enumeration is_required data=*data recursion_level/> }
            },
            _ => {
                unreachable!("Data/type mismatch in EntityInput")
            },
        }
    })
}
