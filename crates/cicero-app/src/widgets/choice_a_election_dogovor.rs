use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ChoiceAElectionDogovor() -> impl IntoView {
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
    let scenarios_names: Vec<String> = scenarios.iter().map(|meta| meta.name.clone()).collect();

    view! {
        <div
            id="choice_a_election_dogovor"
            class="flex flex-col items-start text-start min-h-[200px] w-[550px] bg-[#8C7456] rounded-[10px] p-[10px] gap-[15px]"
        >
<<<<<<< HEAD
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Купля-продажи и мена</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Договоры в сфере корпоративного плана</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Другие договора</a>
                </button>
            </div>
=======
            {scenarios_names
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

>>>>>>> c3e8c97 (Made a display of contract selection, but WIP)
        </div>
    }
}
//<div class="min-h-[35px] w-full text-start">
//<button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px]
//<button min-h-[35px] w-full hover:bg-[#544027]">
//<a href="#">Купля-продажи и мена</a>
//</button>
//</div>
