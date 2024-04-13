use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::{use_params_map, A};

#[component]
pub fn AllSteps(
    #[prop(into)] steps_names: Vec<String>,
    #[prop(into)] pending_step: usize,
) -> impl IntoView {
    let params = use_params_map();
    let scenario_id = move || {
        params
            .with(|params| params.get("id").cloned())
            .and_then(|step| step.parse::<u64>().ok())
            .unwrap_or(0)
    };
    let step_index = move || {
        params
            .with(|params| params.get("step").cloned())
            .and_then(|step| step.parse::<usize>().ok())
            .unwrap_or(0)
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum StepStatus {
        Filled,
        Pending,
        Empty,
    }

    view! {
        {move || {
            steps_names
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let step_status = match i.cmp(&pending_step) {
                        std::cmp::Ordering::Less => StepStatus::Filled,
                        std::cmp::Ordering::Equal => StepStatus::Pending,
                        std::cmp::Ordering::Greater => StepStatus::Empty,
                    };
                    let is_current_step = move || i == step_index();
                    view! {
                        <a href=format!("/scenario/{}/{}", scenario_id(), i)>
                            <button class="hover:bg-[#8C7456] rounded-[10px] font-bold p-[3px] min-h-[40px] max-w-[146px]">
                                <div class="text-[16px] text-[#EEEEEE]">{name.clone()}</div>
                            </button>
                        </a>
                    }
                })
                .collect_view()
        }}
    }
}
