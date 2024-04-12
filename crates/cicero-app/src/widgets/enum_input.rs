use cicero_dsl::types;
use leptos::*;

use crate::shared::data;
use crate::widgets::{EntityInput, HtmlEnumRender, HtmlRender};

#[component]
pub fn EnumInput(
    enumeration: types::Enum,
    is_required: bool,
    data: RwSignal<data::Enum>,
    recursion_level: usize,
) -> impl IntoView {
    view! {
        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
            <div class="flex flex-col gap-[10px] mb-[20px]">

                {
                    let header = enumeration
                        .comment
                        .map(|comment| {
                            view! {
                                <div class="font-bold">
                                    <HtmlRender html_string=comment/>
                                </div>
                            }
                        });
                    let variants = enumeration
                        .variants
                        .clone()
                        .into_values()
                        .zip(data().field)
                        .map(|(enum_var, data_var)| {
                            view! {
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <p>
                                        <HtmlEnumRender html_string=enum_var.name.clone()/>
                                    </p>
                                </div>
                                <EntityInput
                                    entity=type_field.entity
                                    placeholder=type_field.comment
                                    data=data_field
                                    recursion_level=recursion_level + 1
                                />
                            }
                        });
                }

            </div>
        </section>
    }
}
