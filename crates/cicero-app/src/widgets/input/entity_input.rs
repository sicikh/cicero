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

    view! {
        {move || {
            data.with(|data| match (&entity.ty, data) {
                (types::EntityType::Struct(structure), data::Data::Struct(data)) => {
                    view! { <StructInput structure=structure.clone() is_required data=*data recursion_level/> }
                }
                (types::EntityType::String, data::Data::String(data)) => {
                    view! {
                        <StringInput
                            placeholder=placeholder.clone()
                            is_required
                            data=*data
                            _recursion_level=recursion_level
                        />
                    }
                }
                (types::EntityType::Enum(enumeration), data::Data::Enum(data)) => {
                    view! { <EnumInput enumeration=enumeration.clone() is_required data=*data recursion_level/> }
                }
                _ => view! { <p>"Data/type mismatch in EntityInput"</p> }.into_view(),
            })
        }}
    }
}
