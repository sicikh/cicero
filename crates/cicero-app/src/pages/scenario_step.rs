use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::data;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;

use crate::widgets::*;

#[server(GetSteps, "/api", "Url", "get-steps")]
pub async fn get_steps() -> Result<IndexMap<String, Vec<ScenarioStep>>, ServerFnError> {
    let metas = vec![
        ScenarioStep {
            name: "Step1".to_string(),
            header: Some("step_1".to_string()),
            variables: vec![Var {
                name: "UltraDick".to_string(),
                comment: "UltraDick1".to_string(),
                ty: Entity {
                    ty: EntityType::String,
                    is_required: true,
                },
            }],
        },
        ScenarioStep {
            name: "Step2".to_string(),
            header: Some("step_2".to_string()),
            variables: vec![Var {
                name: "UltraDick1".to_string(),
                comment: "UltraDick2".to_string(),
                ty: Entity {
                    ty: EntityType::String,
                    is_required: true,
                },
            }],
        },
    ];
    let map = metas.into_iter().fold(IndexMap::new(), |mut map, meta| {
        map.entry(meta.name.clone())
            .and_modify(|entry: &mut Vec<ScenarioStep>| entry.push(meta.clone()))
            .or_insert(vec![meta]);
        map
    });

    Ok(map)
}

#[component]
pub fn ScenarioStep() -> impl IntoView {
    let selected_steps: RwSignal<Option<ScenarioStep>> = create_rw_signal(None);
    let steps = Resource::once(get_steps);
    view! {
        <Layout>
            <section id="all_page" class="h-full w-full flex flex-row">
                <section
                    id="step"
                    class="pl-[15px] pr-[15px] pt-[15px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                >
                    {move || {
                        steps()
                            .map(move |vari| {
                                match vari {
                                    Ok(steps) => {
                                        let (steps, set_steps) = create_signal(steps);
                                        view! {
                                            <AllSteps steps selected_steps=selected_steps.write_only()/>
                                        }
                                    }
                                    Err(e) => view! { <p>"Error happened"</p> }.into_view(),
                                }
                            })
                    }}

                </section>
                <InputDataStep selected_steps=selected_steps.read_only()/>
                <section
                    id="watch_template"
                    class="flex-1 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]"
                >
                    <div class="w-full h-[45px] border-b-[3px] px-[15px] py-[7px] border-[#8c7456] items-center text-[16px] text-[#8c7456] ">
                        Предварительный просмотр документа
                    </div>
                    <section></section>
                </section>
            </section>
        </Layout>
    }
}
