use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::{StringInput, StructInput};

#[component]
pub fn EntityInput(
    entity: types::Entity,
    placeholder: String,
    data: RwSignal<data::Data>,
) -> impl IntoView {
    let is_required = entity.is_required;
    // CHECK: closure?
    data.with(|data| {
        match (entity.ty, data) {
            (types::EntityType::Struct(structure), data::Data::Struct(data)) => {
                view! { <StructInput structure is_required data=*data/> }
            },
            (types::EntityType::String, data::Data::String(data)) => {
                view! { <StringInput placeholder is_required data=*data/> }
            },
            _ => {
                unreachable!("Data/type mismatch in EntityInput")
            },
        }
    })
}
