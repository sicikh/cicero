use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn AllSteps(#[prop(into)] steps: Signal<IndexMap<String, Vec<ScenarioStep>>>) -> impl IntoView {
    let selected_step: RwSignal<String> =
        create_rw_signal(steps.with(|inner| inner.keys().next().unwrap().clone()));

    view! {
        {move || {
            steps
                .with(|key| {
                    key.keys()
                        .map(|name_step| {
                            view! {
                                <button
                                    class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]"
                                    on:click=move |mv| {
                                        let button: web_sys::HtmlButtonElement = event_target(&mv);
                                        let name = button.inner_text();
                                        selected_step.set(name);
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
