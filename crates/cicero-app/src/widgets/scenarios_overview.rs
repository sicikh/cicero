use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::ScenarioMeta;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ScenariosOverview(
    #[prop(into)] scenarios: Signal<IndexMap<String, Vec<ScenarioMeta>>>,
    selected_scenario: WriteSignal<Option<ScenarioMeta>>,
    #[prop(into)] filter: Signal<String>,
) -> impl IntoView {
    let selected_category: RwSignal<String> =
        create_rw_signal(scenarios.with_untracked(|inner| inner.keys().next().unwrap().clone()));

    view! {
        <section class="flex flex-row justify-evenly mt-[25px]">
            {move || {
                if filter.with(String::is_empty) {
                    view! {
                        <div
                            id="choice_a_dogovor"
                            class="flex flex-col min-h-[145px] w-[280px] bg-[#8C7456] items-start text-start rounded-[10px] p-[10px] gap-[15px]"
                        >

                            {move || {
                                scenarios
                                    .with(|inner| {
                                        inner
                                            .keys()
                                            .map(|category| {
                                                view! {
                                                    <div class="min-h-[35px] w-full text-start">
                                                        <button
                                                            class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]"
                                                            on:click=move |ev| {
                                                                let button: web_sys::HtmlButtonElement = event_target(&ev);
                                                                let name = button.inner_text();
                                                                selected_category.set(name);
                                                            }
                                                        >

                                                            {category}
                                                        </button>
                                                    </div>
                                                }
                                            })
                                            .collect_view()
                                    })
                            }}

                        </div>
                        <div
                            id="choice_a_election_dogovor"
                            class="flex flex-col items-start text-start min-h-[200px] w-[550px] bg-[#8C7456] rounded-[10px] p-[10px] gap-[15px]"
                        >

                            {move || {
                                scenarios
                                    .with(|inner| {
                                        inner
                                            .get(&selected_category())
                                            .map(|category_scenarios| {
                                                category_scenarios
                                                    .iter()
                                                    .map(|scenario| {
                                                        view! {
                                                            <div class="min-h-[35px] w-full text-start">
                                                                <button
                                                                    class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]"
                                                                    on:click=move |ev| {
                                                                        let button: web_sys::HtmlButtonElement = event_target(&ev);
                                                                        let name = button.inner_text();
                                                                        let scenario = scenarios
                                                                            .with(|inner| {
                                                                                inner
                                                                                    .get(&selected_category())
                                                                                    .unwrap()
                                                                                    .iter()
                                                                                    .find(|&scenario| scenario.name == name)
                                                                                    .cloned()
                                                                            });
                                                                        selected_scenario.set(scenario);
                                                                    }
                                                                >

                                                                    {scenario.name.clone()}
                                                                </button>
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()
                                            })
                                            .collect_view()
                                    })
                            }}

                        </div>
                    }
                        .into_view()
                } else {
                    scenarios
                        .with(|inner| {
                            inner
                                .values()
                                .flatten()
                                .filter(|&scenario| {
                                    filter
                                        .with(|inner| { scenario.name.to_lowercase().contains(&inner.to_lowercase()) })
                                })
                                .map(|scenario| {
                                    view! {
                                        <div class="min-h-[35px] w-full text-start">
                                            <button
                                                class="text-[#000000] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]"
                                                on:click=move |ev| {
                                                    let button: web_sys::HtmlButtonElement = event_target(&ev);
                                                    let name = button.inner_text();
                                                    let scenario = scenarios
                                                        .with(|inner| {
                                                            inner
                                                                .values()
                                                                .flatten()
                                                                .find(|&scenario| scenario.name == name)
                                                                .cloned()
                                                        });
                                                    selected_scenario.set(scenario);
                                                }
                                            >

                                                {scenario.name.clone()}
                                            </button>
                                        </div>
                                    }
                                })
                                .collect_view()
                        })
                }
            }}

        </section>
    }
}
