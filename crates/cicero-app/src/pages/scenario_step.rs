use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::data;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;

use crate::widgets::*;

#[server(GetStepsNames, "/api", "Url", "get-steps-names")]
pub async fn get_steps_names() -> Result<Vec<String>, ServerFnError> {
    let names = vec!["Преамбула", "Основная часть договора"].into_iter().map(String::from).collect();

    Ok(names)
}

#[server(GetScenarioStep, "/api", "Url", "get-scenario-step")]
pub async fn get_scenario_step() -> Result<ScenarioStep, ServerFnError> {
    let step = ScenarioStep {
        name: "Преамбула".to_string(),
        header: Some("Комментарий **жирное начертание**, *курсив*, [ссылка](https://vk.com).".to_string()),
        variables: vec![Var {
            name: "person_name".to_string(),
            comment: "Введите имя пользователя".to_string(),
            ty: Entity {
                ty: EntityType::String,
                is_required: true,
            },
        }],
    };

    Ok(step)
}

#[component]
pub fn ScenarioStep() -> impl IntoView {
    let step_index: RwSignal<usize> = create_rw_signal(0);
    let current_step = create_resource(step_index, move |_| async { get_scenario_step().await });
    let steps_names = Resource::once(get_steps_names);
    view! {
        <Layout>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=move |_| {
                    view! { <p>"Error happened"</p> }
                }>

                    {move || {
                        match (steps_names(), current_step()) {
                            (Some(Ok(steps_names)), Some(Ok(current_step))) => {
                                let (current_step, _) = create_signal(current_step);
                                let (steps_names, _) = create_signal(steps_names);
                                view! {
                                    <section id="all_page" class="h-full w-full flex flex-row">
                                        <section
                                            id="step"
                                            class="pl-[15px] pr-[15px] pt-[15px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                                        >
                                            <AllSteps steps_names step_index/>
                                        </section>
                                        <InputDataStep current_step step_index/>
                                        <section class="flex-1 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]">
                                            <div class="w-full h-[45px] border-b-[3px] px-[15px] py-[7px] border-[#8c7456] items-center text-[16px] text-[#8c7456] ">
                                                Предварительный просмотр документа
                                            </div>
                                            <section></section>
                                        </section>
                                    </section>
                                }
                                    .into_view()
                            }
                            (_, _) => view! { <p>"Error happened"</p> }.into_view(),
                        }
                    }}

                </ErrorBoundary>
            </Transition>
        </Layout>
    }
}
