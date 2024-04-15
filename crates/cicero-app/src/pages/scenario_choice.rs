/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>,
 * Gleb Krylov <gleb_cry@mail.ru>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::collections::HashMap;
use std::hash::Hash;

use cfg_if::cfg_if;
use cicero_dsl::data;
use cicero_dsl::types::ScenarioMeta;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;

use crate::api::{UserId, UserPassword};
use crate::widgets::*;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use crate::shared::server::Env;
    }
);

#[server(GetScenarios, "/api", "Url", "get-scenarios")]
pub async fn get_scenarios_metas() -> Result<IndexMap<String, Vec<ScenarioMeta>>, ServerFnError> {
    let env = Env::from_context()?;

    let metas = (*env.scenarios_metas).clone();

    Ok(metas)
}

#[component]
pub fn ScenarioChoice() -> impl IntoView {
    let selected_scenario: RwSignal<Option<ScenarioMeta>> = create_rw_signal(None);
    let filter: RwSignal<String> = create_rw_signal(String::new());

    let scenarios = Resource::once(get_scenarios_metas);

    view! {
        <Title text="Выбор шаблона"/>
        <Layout>
            <div id="main_body" class="flex flex-1 flex-row h-[calc(100vh-180px)] bg-[#EEEEEE]">
                <div id="left_side" class="md:flex flex-col flex-1 border-r-[7px] border-[#8C7456]">
                    <SearchBar filter/>
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        <ErrorBoundary fallback=move |_| {
                            view! { <p>"Error happened"</p> }
                        }>

                            {move || {
                                scenarios()
                                    .map(move |data| {
                                        match data {
                                            Ok(scenarios) => {
                                                let (scenarios, _set_scenarios) = create_signal(scenarios);
                                                view! {
                                                    <ScenariosOverview
                                                        scenarios
                                                        selected_scenario=selected_scenario.write_only()
                                                        filter=filter.read_only()
                                                    />
                                                }
                                            }
                                            Err(e) => view! { <p>"Error happened"</p> }.into_view(),
                                        }
                                    })
                            }}

                        </ErrorBoundary>
                    </Suspense>
                </div>
                <ScenarioDescription selected_scenario=selected_scenario.read_only()/>
            </div>
        </Layout>
    }
}
