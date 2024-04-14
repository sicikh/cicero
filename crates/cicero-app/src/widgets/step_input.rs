use std::collections::HashMap;

use cfg_if::cfg_if;
use cicero_dsl::data as dsl;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::html::P;
use leptos::*;
use leptos_meta::*;
use leptos_router::{use_params_map, Form, A};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

use crate::api::{ScenarioId, User, UserId, UserPassword};
use crate::data::data_from_entity;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::server::Env;
    }
}

use crate::shared::data;
use crate::widgets::{EntityInput, HtmlRender};

#[server(PutStepData, "/api", "Url", "put-step-data")]
pub async fn put_step_data(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
    step_id: usize,
    data: Vec<(String, dsl::Var)>,
) -> Result<usize, ServerFnError> {
    let env = Env::from_context()?;

    logging::log!("put_step_data: {:?}", data);

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    };

    let data = data.into_iter().collect::<HashMap<String, dsl::Var>>();

    env.insert_data(user_id, scenario_id, step_id, data).await
}

#[allow(clippy::match_single_binding)] // check fixme comment below
#[component]
pub fn StepInput(
    scenario_step: ScenarioStep,
    var_data: Option<HashMap<String, dsl::Var>>,
    signal: RwSignal<Option<usize>>,
) -> impl IntoView {
    let (user, ..) = use_local_storage::<User, JsonCodec>("user");

    let var_data = create_rw_signal({
        match var_data {
            // FIXME: invalid position of data leading to Data/Type mismatch in entity input
            // Some(var_data) => scenario_step.variables.into_iter().map(|var| {
            //     let data = RwSignal::new(var_data.get(&var.name).cloned().map(|var|
            // var.data.into()).unwrap_or_else(|| data_from_entity(&var.ty.ty)));
            //     (var, data)
            // }).collect::<Vec<(Var, RwSignal<data::Data>)>>(),
            _ => {
                scenario_step
                    .variables
                    .into_iter()
                    .map(|var| {
                        let data = RwSignal::new(data_from_entity(&var.ty.ty));
                        (var, data)
                    })
                    .collect::<Vec<(Var, RwSignal<data::Data>)>>()
            },
        }
    });
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
    let result = create_resource(
        move || {
            (
                signal(),
                var_data.get_untracked(),
                user.get_untracked(),
                scenario_id(),
                step_index(),
            )
        },
        |(signal, var_data, user, scenario_id, step_index)| {
            async move {
                match signal {
                    None => None,
                    Some(step_id) => {
                        let data = var_data
                            .into_iter()
                            .map(|(var, data)| {
                                (var.name.clone(), dsl::Var {
                                    name: var.name.clone(),
                                    data: data().into(),
                                })
                            })
                            .collect::<Vec<(String, dsl::Var)>>();
                        Some(
                            put_step_data(user.id, user.password, scenario_id, step_index, data)
                                .await,
                        )
                    },
                }
            }
        },
    );

    view! {
        <section id="input_data" class="flex flex-col flex-1 h-[100%] bg-[#EEEEEE] border-r-[7px] border-[#8c7456]">
            {move || {
                let header = if let Some(html_string) = scenario_step.header.clone() {
                    view! {
                        <section
                            id="header"
                            class="min-h-[80px] bg-[#cccccc] border-b-[3px] px-[15px] gap-[2px] items-start flex flex-col border-[#8C7456]"
                        >
                            <div class="text-[15px] text-[#8C7456] items-center">
                                <HtmlRender html_string/>
                            </div>
                        </section>
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                };
                let input = var_data
                    .with(|var_data| {
                        var_data
                            .iter()
                            .map(|(var, data)| {
                                view! {
                                    <section class="flex flex-col text-[#8c7456] pt-[5px] w-full px-[15px] pb-[10px]">
                                        <div class="flex flex-col gap-[10px] mb-[20px]">
                                            <div class="font-bold">
                                                <HtmlRender html_string=var.comment.clone()/>
                                            </div>
                                            <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                                <EntityInput
                                                    entity=var.ty.clone()
                                                    placeholder=var.comment.clone()
                                                    data=*data
                                                    recursion_level=0
                                                />
                                            </div>
                                        </div>
                                    </section>
                                }
                            })
                            .collect_view()
                    });
                let button = view! {
                    <button
                        class="bg-[#8C7456] text-[#FFFFFF] mb-[15px] py-[5px] mx-[20px] h-[80px] text-[18px] rounded-[15px] items-center mt-[10px] hover:shadow-2xl hover:text-[#FFFFFF]"
                        on:click=move |_| {
                            signal
                                .update(|signal| {
                                    *signal = match signal {
                                        None => Some(0),
                                        Some(signal) => Some(*signal + 1),
                                    };
                                });
                        }
                    >

                        Применить изменения
                    </button>
                };
                (header, input, button)
            }}

        </section>
    }
}
