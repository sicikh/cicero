use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::html::P;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Form, A};

use crate::data::data_from_entity;
use crate::widgets::{EntityInput, HtmlRender};

#[component]
pub fn StepInput(scenario_step: ScenarioStep) -> impl IntoView {
    view! {
        <section id="input_data" class="flex flex-col flex-1 h-[100%] bg-[#EEEEEE] border-r-[7px] border-[#8c7456]">
            {move || {
                let header = if let Some(html_string) = scenario_step.header.clone() {
                    view! {
                        <section
                            id="header"
                            class="min-h-[80px] bg-[#cccccc] border-b-[3px] py-[15px] px-[15px] pb-[25px] gap-[2px] items-start flex flex-col border-[#8C7456]"
                        >
                            <div class="text-[15px] text-[#8C7456] items-center">
                                <HtmlRender html_string/>
                            </div>
                        </section>
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                };
                let input = scenario_step
                    .variables
                    .iter()
                    .map(|var| {
                        let data = RwSignal::new(data_from_entity(&var.ty.ty));
                        view! {
                            // TODO: move data into hashmap<var_name(String), RwSignal<Data>>
                            <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                                <div class="flex flex-col gap-[10px] mb-[20px]">
                                    <div class="font-bold">
                                        <HtmlRender html_string=var.comment.clone()/>
                                    </div>
                                    <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                        <EntityInput
                                            entity=var.ty.clone()
                                            placeholder="".to_string()
                                            data
                                            recursion_level=0
                                        />
                                    </div>
                                </div>
                            </section>
                        }
                    })
                    .collect_view();
                (header, input)
            }}

        </section>
    }
}
