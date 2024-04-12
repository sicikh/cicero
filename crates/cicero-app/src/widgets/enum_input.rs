use cicero_dsl::types;
use leptos::*;

use crate::data::data_from_entity;
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
                                .into_view()
                        });
                    let variants = enumeration
                        .variants
                        .clone()
                        .into_values()
                        .map(|enum_var| {
                            view! {
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <p>
                                        <HtmlEnumRender html_string=enum_var.name.clone()/>
                                    </p>
                                </div>

                                {enum_var
                                    .field
                                    .map(|field| {
                                        let data_signal = RwSignal::new(data_from_entity(&field.ty));
                                        data.update(|data| {
                                            data.field = Some(data_signal);
                                        });
                                        view! {
                                            <EntityInput
                                                entity=field
                                                placeholder=enum_var.comment.clone()
                                                data=data_signal
                                                recursion_level=recursion_level + 1
                                            />
                                        }
                                            .into_view()
                                    })}
                            }
                                .into_view()
                        })
                        .collect_view();
                    (header, variants)
                }

            </div>
        </section>
    }
}
