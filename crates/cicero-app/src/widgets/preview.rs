use leptos::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

use crate::api::User;

#[component]
pub fn Preview(urls: Vec<String>) -> impl IntoView {
    let (user, ..) = use_local_storage::<User, JsonCodec>("user");

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

    view! {
        {move || {
            urls.iter()
                .map(|url| {
                    view! {
                        <img
                            src=format!("/data/{}/{}/{}/{}", user().id, scenario_id(), step_index(), url)

                            class="m-[10px]"
                        />
                    }
                })
                .collect_view()
        }}
    }
}
