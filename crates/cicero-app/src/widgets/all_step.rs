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
    selected_step: ReadSignal<Option<ScenarioStep>>,
) -> impl IntoView {
    view! {
        {move || {
            steps
                .with(|key| {
                    let keys = key.keys().cloned().collect::<Vec<_>>();
                    keys.into_iter()
                        .map(|name_step| {
                            let link = format!("/{}", name_step);
                            view! {
                                <A href=link.clone()>
                                    <button
                                        class="hover:bg-[#696764] rounded-[10px] h-[40px] w-[100px]"
                                        on:click=move |mv| {
                                            let button: web_sys::HtmlButtonElement = event_target(&mv);
                                            let name = button.inner_text();
                                        }
                                    >

                                        <div class="text-[24px] text-[#EEEEEE]">{name_step}</div>
                                    </button>
                                </A>
                            }
                        })
                        .collect::<Vec<_>>()
                })
        }}
    }
}
