use std::collections::HashMap;
use std::hash::Hash;

use cfg_if::cfg_if;
use cicero_dsl::data as dsl;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::{use_navigate, use_params_map, NavigateOptions, A};
use leptos_use::storage::use_local_storage;
use leptos_use::use_window;
use leptos_use::utils::JsonCodec;
use serde::Deserialize;

use crate::shared::api::{ScenarioId, User, UserId, UserPassword};
use crate::widgets::{AllSteps, Layout, Preview, StepInput};

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use crate::shared::server::Env;
    }
);

// This function does too much, but this is for sake of simplicity on the client
// side.
#[server(StartOrContinueScenario, "/api", "Url", "start-or-continue-scenario")]
pub async fn start_or_continue_scenario(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
    desired_step_id: usize,
) -> Result<
    (
        ScenarioStep,
        usize,
        Vec<String>,
        Option<HashMap<String, dsl::Var>>,
    ),
    ServerFnError,
> {
    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    }

    let data = env
        .start_or_continue_scenario(user_id, scenario_id, desired_step_id)
        .await;

    data.ok_or_else(|| ServerFnError::ServerError("Could not start scenario".to_string()))
}

/// Render scenario step.
///
/// Returns a list of urls to the rendered images, that are available at
/// "/static/rendered/{user_id}/{scenario_id}/{step}/{url}".
#[server(RenderScenarioStep, "/api", "Url", "render-scenario-step")]
pub async fn render_scenario_step(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
    step_id: usize,
) -> Result<Vec<String>, ServerFnError> {
    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    }

    let urls = env
        .render_scenario_step(user_id, scenario_id, step_id)
        .await;

    urls.ok_or_else(|| ServerFnError::ServerError("Could not render scenario step".to_string()))
}

// TODO: Maybe use streaming instead of returning url. Find a way to do this in
// Leptos.
#[server(FullRenderScenarioPdf, "/api", "Url", "full-render-scenario-pdf")]
pub async fn full_render_scenario_pdf(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
) -> Result<String, ServerFnError> {
    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    }

    let url = env
        .full_render_pdf(user_id, scenario_id)
        .await
        .map(|path| path.to_string_lossy().to_string());

    url.ok_or_else(|| ServerFnError::ServerError("Could not render scenario".to_string()))
}

#[server(FullRenderScenarioDocx, "/api", "Url", "full-render-scenario-docx")]
pub async fn full_render_scenario_docx(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
) -> Result<String, ServerFnError> {
    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    }

    let url = env
        .full_render_docx(user_id, scenario_id)
        .await
        .map(|path| path.to_string_lossy().to_string());

    url.ok_or_else(|| ServerFnError::ServerError("Could not render scenario".to_string()))
}

#[server(Register, "/api", "Url", "register")]
pub async fn register() -> Result<(UserId, UserPassword), ServerFnError> {
    let env = Env::from_context()?;

    let user_data = env.register_user().await;

    Ok(user_data)
}

#[server(Login, "/api", "Url", "login")]
pub async fn login(user_id: UserId, user_password: UserPassword) -> Result<bool, ServerFnError> {
    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    Ok(is_logged_in)
}

// This code cost me 5 hours of debugging and I still don't know why it works
// and why it doesn't work. Currently it sends from 4 to 5 requests to the
// server, but it should send only from 1 to 2. Simply retrieve user from local
// storage, if password is empty, then register, otherwise try to login and if
// it fails, register. but in reactive world my opinion is not yet been
// initialized!!!
#[component]
pub fn ScenarioStep() -> impl IntoView {
    let user = create_local_resource(
        move || (),
        move |()| {
            async move {
                let window = window();
                let storage = window.local_storage().unwrap().unwrap();

                let user = storage
                    .get_item("user")
                    .unwrap()
                    .map(|json| serde_json::from_str::<User>(&json).unwrap());

                match user {
                    None => {
                        let (user_id, user_password) = register().await.unwrap();
                        storage
                            .set_item(
                                "user",
                                serde_json::to_string(&User {
                                    id: user_id,
                                    password: user_password.clone(),
                                })
                                .unwrap()
                                .as_str(),
                            )
                            .unwrap();
                        (user_id, user_password.clone())
                    },
                    Some(user) => {
                        let is_logged_in = login(user.id, user.password.clone()).await.unwrap();
                        if !is_logged_in {
                            let (user_id, user_password) = register().await.unwrap();
                            storage
                                .set_item(
                                    "user",
                                    serde_json::to_string(&User {
                                        id: user_id,
                                        password: user_password.clone(),
                                    })
                                    .unwrap()
                                    .as_str(),
                                )
                                .unwrap();
                            (user_id, user_password)
                        } else {
                            (user.id, user.password.clone())
                        }
                    },
                }
            }
        },
    )
    .into_signal();

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
    let signal = create_rw_signal(None);

    let data = create_resource(
        move || {
            user().map(move |(user_id, user_password)| {
                (user_id, user_password, scenario_id(), step_index())
            })
        },
        |opt| {
            async move {
                match opt {
                    None => None,
                    Some((user_id, user_password, scenario_id, step_index)) => {
                        start_or_continue_scenario(user_id, user_password, scenario_id, step_index)
                            .await
                            .ok()
                    },
                }
            }
        },
    );

    let pngs = create_resource(
        move || (signal(), user.get_untracked(), scenario_id(), step_index()),
        |(signal, user, scenario_id, step_index)| {
            async move {
                match (signal, user) {
                    (Some(i), Some((user_id, user_password))) if i > 0 => {
                        Some(
                            render_scenario_step(user_id, user_password, scenario_id, step_index)
                                .await,
                        )
                    },
                    _ => None,
                }
            }
        },
    );

    view! {
        <Layout>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=move |_| {
                    view! { <p>"Error happened"</p> }
                }>
                    {move || {
                        user();
                    }}
                    {move || {
                        match data() {
                            Some(Some((scenario_step, pending_step, steps_names, var_data))) => {
                                let scenario_step_index = steps_names
                                    .iter()
                                    .position(|name| name == &scenario_step.name)
                                    .unwrap();
                                if scenario_step_index != step_index() {
                                    let navigate = use_navigate();
                                    navigate(
                                        format!(
                                            "/scenario/{}/{}",
                                            scenario_id(),
                                            scenario_step_index,
                                        )
                                            .as_str(),
                                        Default::default(),
                                    );
                                }
                                view! {
                                    <section
                                        id="all_page"
                                        class="h-[calc(100vh-180px)] w-full flex flex-row overflow-hidden"
                                    >
                                        <section
                                            id="step"
                                            class="p-[10px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                                        >
                                            <AllSteps steps_names pending_step/>
                                        </section>
                                        <StepInput scenario_step signal var_data/>
                                        <section class="flex-1 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]">
                                            <div class="w-full h-[45px] border-b-[3px] px-[15px] py-[7px] border-[#8c7456] items-center text-[16px] text-[#8c7456] ">
                                                Предварительный просмотр документа
                                            </div>
                                            <section>
                                                {move || {
                                                    match pngs() {
                                                        Some(Some(Ok(urls))) => Some(view! { <Preview urls/> }),
                                                        _ => None,
                                                    }
                                                }}

                                            </section>
                                        </section>
                                    </section>
                                }
                                    .into_view()
                            }
                            _ => view! {}.into_view(),
                        }
                    }}

                </ErrorBoundary>
            </Transition>
        </Layout>
    }
}
