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
                        .map(|enum_var| {
                            view! {
                                // Add index here
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">

                                    {
                                        let id = enumeration.name.clone();
                                        view! {
                                            <StringEnumInput
                                                html_string=enum_var.comment.clone()
                                                id=id.clone()
                                                name=name.clone()
                                            />
                                        }
                                    }

                                </div>

                                {move || {
                                    enum_var
                                        .field
                                        .as_ref()
                                        .map(|field| {
                                            let data_signal = RwSignal::new(
                                                data_from_entity(&field.ty),
                                            );
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
                                        })
                                }}
                            }
                        })
                        .collect_view();
                    (
                        header,
                        view! {
                            // Add index here

                            // Add index here

                            // Add index here

                            // Add index here

                            <form>{variants}</form>
                        },
                    )
                }}

            </div>
        </section>
    }
}
