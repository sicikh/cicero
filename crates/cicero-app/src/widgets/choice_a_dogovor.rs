use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ChoiceADogovor() -> impl IntoView {
    let metas = vec![ScenarioMeta {
        id: 0,
        name: "Test".to_string(),
        description: "Test".to_string(),
        category: "Testing".to_string(),
    }];

    let categories: HashMap<String, Vec<ScenarioMeta>> =
        metas.into_iter().fold(HashMap::new(), |mut map, meta| {
            map.entry(meta.category.clone())
                .and_modify(|entry| entry.push(meta.clone()))
                .or_insert(vec![meta]);
            map
        });

    let scenarios = categories.get(&"Testing".to_string()).unwrap().to_vec();
    let scenarios_category: Vec<String> =
        scenarios.iter().map(|meta| meta.category.clone()).collect();

    view! {
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
    }
}
