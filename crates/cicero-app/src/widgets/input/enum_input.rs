use cicero_dsl::types;
use leptos::*;

use crate::data::data_from_entity;
use crate::shared::data;
use crate::widgets::{EntityInput, HtmlEnumRender, HtmlRender, StringEnumInput};

#[component]
pub fn EnumInput(
    enumeration: types::Enum,
    is_required: bool,
    data: RwSignal<data::Enum>,
    recursion_level: usize,
    name: String,
) -> impl IntoView {
    let selected = create_rw_signal(0usize);
    let effect_enumeration = enumeration.clone();
    create_effect(move |_| {
        let (_, variant) = effect_enumeration
            .variants
            .values()
            .enumerate()
            .find(|(i, _)| *i == selected())
            .unwrap(); // panics if selected is out of bounds
        data.update(|data| {
            data.discriminant.clone_from(&variant.name);
        });
    });

    view! {
        <section class="flex flex-col text-[#8c7456] w-full pr-[15px]">
            <div class="flex flex-col gap-[10px]">

                {move || {
                    let header = enumeration
                        .comment
                        .as_ref()
                        .map(|comment| {
                            view! {
                                <div class="font-bold">
                                    <HtmlRender html_string=comment.clone()/>
                                </div>
                            }
                        });
                    let variants = enumeration
                        .variants
                        .clone()
                        .into_values()
                        .enumerate()
                        .map(|(i, enum_var)| {
                            view! {
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">

                                    {
                                        let id = enumeration.name.clone();
                                        view! {
                                            <StringEnumInput
                                                html_string=enum_var.comment.clone()
                                                id=id.clone()
                                                name=name.clone()
                                                value=i
                                                selected=selected.write_only()
                                            />
                                        }
                                    }

                                </div>

                                {move || {
                                    enum_var
                                        .field
                                        .as_ref()
                                        .map(|field| {
                                            if selected() == i {
                                                let data_signal = RwSignal::new(data_from_entity(&field.ty));
                                                data.clone()
                                                    .update(|data| {
                                                        data.field = Some(data_signal);
                                                    });
                                                view! {
                                                    <EntityInput
                                                        entity=field.clone()
                                                        placeholder=enum_var.comment.clone()
                                                        data=data_signal
                                                        recursion_level=recursion_level + 1
                                                    />
                                                }
                                            } else {
                                                view! {}.into_view()
                                            }
                                        })
                                }}
                            }
                        })
                        .collect_view();
                    (header, view! { <form>{variants}</form> })
                }}

            </div>
        </section>
    }
}
