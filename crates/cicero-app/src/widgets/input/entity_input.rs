use std::sync::atomic::{AtomicUsize, Ordering};

use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::{ArrayInput, EnumInput, StringInput, StructInput};
static COUNTER: AtomicUsize = AtomicUsize::new(0);

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
                    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
                    view! {
                        <EnumInput
                            enumeration=enumeration.clone()
                            name=id.to_string()
                            is_required
                            data=*data
                            recursion_level
                        />
                    }
                }
                (types::EntityType::Array(array), data::Data::Array(data)) => {
                    view! { <ArrayInput array=*array.clone() is_required data=*data recursion_level/> }
                }
                _ => {
                    view! { <p>{format!("Data/type mismatch in EntityInput: {:?} and {:?}", &entity, &data)}</p> }
                        .into_view()
                }
            })
        }}
    }
}

/*
(types::EntityType::PhoneNumber) => {
    view! {
        <PhoneNumberInput
            placeholder=placeholder.clone()
            is_required
            recursion_level=recursion_level
        />
    }
}
(types::EntityType::Place) => {
    view! {
        <PlaceInput
            placeholder=placeholder.clone()
            is_required
            recursion_level=recursion_level
        />
    }
}*/
