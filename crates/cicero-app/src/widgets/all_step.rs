use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;

#[component]
pub fn AllSteps(
    #[prop(into)] steps: Signal<IndexMap<String, Vec<ScenarioStep>>>,
    selected_steps: WriteSignal<Option<ScenarioStep>>,
) -> impl IntoView {
    let selected_step: RwSignal<String> =
        create_rw_signal(steps.with(|sel| sel.keys().next().unwrap().clone()));
    view! {
        {move || {
            steps
                .with(|key| {
                    let keys = key.keys().cloned().collect::<Vec<_>>();
                    keys.into_iter()
                        .map(|name_step| {
                            let link = format!("{}", name_step);
                            view! {
                                // <A href=link.clone()>
                                <button
                                    class="hover:bg-[#696764] rounded-[10px] h-[40px] w-[100px]"
                                    on:click=move |mv| {
                                        let button: web_sys::HtmlButtonElement = event_target(&mv);
                                        let name = button.inner_text();
                                        let step = steps
                                            .with(|inner| {
                                                inner
                                                    .get(&selected_step())
                                                    .unwrap()
                                                    .iter()
                                                    .find(|&step| step.name == name)
                                                    .cloned()
                                            });
                                        selected_steps.set(step);
                                    }
                                >

                                    <div class="text-[24px] text-[#EEEEEE]">{name_step}</div>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                })
        }}
    }
}
