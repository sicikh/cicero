use std::collections::HashMap;

use cicero_dsl::data as dsl;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::html::P;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Form, A};

use crate::api::{ScenarioId, UserId, UserPassword};
use crate::data::data_from_entity;
use crate::shared::data;
use crate::widgets::{EntityInput, HtmlRender};

#[server(PutStepData, "/api", "Url", "put-step-data")]
pub async fn put_step_data(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
    step_id: usize,
    data: Vec<(String, dsl::Data)>,
) -> Result<(), ServerFnError> {
    logging::log!("put_step_data: {:?}", data);

    Ok(())
}

#[component]
pub fn StepInput(scenario_step: ScenarioStep) -> impl IntoView {
    let var_data = create_rw_signal(
        scenario_step
            .variables
            .into_iter()
            .map(|var| {
                let data = RwSignal::new(data_from_entity(&var.ty.ty));
                (var, data)
            })
            .collect::<Vec<(Var, RwSignal<data::Data>)>>(),
    );

    view! {
        <section
            id="input_data"
            class="flex flex-col flex-1 h-[100%] bg-[#EEEEEE] border-r-[7px] border-[#8c7456]"
        >
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
                let input = var_data
                    .with(|var_data| {
                        var_data
                            .iter()
                            .map(|(var, data)| {
                                view! {
                                    <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                                        <div class="flex flex-col gap-[10px] mb-[20px]">
                                            <div class="font-bold">
                                                <HtmlRender html_string=var.comment.clone()/>
                                            </div>
                                            <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                                <EntityInput
                                                    entity=var.ty.clone()
                                                    placeholder=var.comment.clone()
                                                    data=*data
                                                    recursion_level=0
                                                />
                                            </div>
                                        </div>
                                    </section>
                                }
                            })
                            .collect_view()
                    });
                let button = view! {
                    <button
                        class="bg-[#8C7456] text-[#FFFFFF] py-[10px] px-[20px] rounded-[5px] mt-[20px] hover:bg-[#8C7456] hover:text-[#FFFFFF]"
                        on:click=move |_| {
                            let data = var_data()
                                .into_iter()
                                .map(|(var, data)| { (var.name.clone(), data().into()) })
                                .collect::<Vec<(String, dsl::Data)>>();
                            spawn_local(async {
                                put_step_data(0, "".to_string(), 0, 0, data).await.unwrap();
                            });
                        }
                    >

                        <p>submit</p>
                    </button>
                };
                (header, input, button)
            }}

        </section>
    }
}
