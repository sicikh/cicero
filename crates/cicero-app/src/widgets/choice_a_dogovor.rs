use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;


#[server(GetScenarios, "/api", "Url", "get-scenarios")]
pub async fn get_scenarios() -> Result<Vec<ScenarioMeta>, ServerFnError> {
    let metas = vec![ScenarioMeta {
        id: 0,
        name: "Test".to_string(),
        description: "Test".to_string(),
        category: "Testing".to_string(),
    }];
    Ok(metas)
}

#[component]
pub fn ChoiceADogovor() -> impl IntoView {
    let metas = create_resource(|| (), |_| async move { get_scenarios().await });
    let categories = move || {
        metas.with(|inner| match inner {
            Some(Ok(metas)) => metas.into_iter().cloned().fold(HashMap::new(), |mut map, meta| {
                map.entry(meta.category.clone())
                    .and_modify(|entry:&mut Vec<ScenarioMeta>| entry.push(meta.clone()))
                    .or_insert(vec![meta]);
                map
            }),  
            Some(Err(error)) => todo!(),
            None => todo!(),
        })
    };
    
    let scenarios = categories().get(&"Testing".to_string()).unwrap().to_vec();
    let scenarios_category: Vec<String> =
        scenarios.iter().map(|meta| meta.category.clone()).collect();
    let scenarios_name: Vec<String> = scenarios.iter().map(|meta| meta.name.clone()).collect();

    let (scenarios_names, set_scenarios_names) = create_signal(0);

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <div
                id="choice_a_dogovor"
                class="flex flex-col min-h-[145px] w-[280px] bg-[#8C7456] items-start text-start rounded-[10px] p-[10px] gap-[15px]"
            >
                {scenarios_category
                    .into_iter()
                    .map(|category| {
                        view! {
                            <div class="min-h-[35px] w-full text-start">
                                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                                    <a href="#">{category}</a>
                                </button>
                            </div>
                        }
                    })
                    .collect_view()}

            </div>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <div
                id="choice_a_election_dogovor"
                class="flex flex-col items-start text-start min-h-[200px] w-[550px] bg-[#8C7456] rounded-[10px] p-[10px] gap-[15px]"
            >
                {scenarios_name
                    .into_iter()
                    .map(|name| {
                        view! {
                            <div class="min-h-[35px] w-full text-start">
                                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                                    <a href="#">{name}</a>
                                </button>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </Suspense>
    }
}
