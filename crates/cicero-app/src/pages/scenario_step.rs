use std::collections::HashMap;
use std::hash::Hash;

use cfg_if::cfg_if;
use cicero_dsl::data as dsl;
use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::{use_navigate, use_params_map, A};

use crate::shared::api::{ScenarioId, UserId, UserPassword};
use crate::widgets::*;

cfg_if!(
    if #[cfg(feature = "ssr")] {
        use crate::shared::server::Env;
    }
);

#[server(GetStepsNames, "/api", "Url", "get-steps-names")]
pub async fn get_steps_names() -> Result<Vec<String>, ServerFnError> {
    let names = vec!["Преамбула", "Основная часть договора"]
        .into_iter()
        .map(String::from)
        .collect();

    Ok(names)
}

#[server(GetScenarioStep, "/api", "Url", "get-scenario-step")]
pub async fn get_scenario_step() -> Result<ScenarioStep, ServerFnError> {
    let passport_struct = Struct {
        name: "Passport".to_string(),
        comment: Some("<p>Комментарий паспорта</p>".to_string()),
        fields: {
            let mut fields = IndexMap::new();
            fields.insert("series".to_string(), Field {
                comment: "<p>Серия:</p>".to_string(),
                entity: Entity {
                    ty: EntityType::String,
                    is_required: true,
                },
            });
            fields.insert("number".to_string(), Field {
                comment: "<p>Номер:</p>".to_string(),
                entity: Entity {
                    ty: EntityType::String,
                    is_required: true,
                },
            });
            fields.insert("bobr".to_string(), Field {
                comment: "<p>Бобр:</p>".to_string(),
                entity: Entity {
                    ty: EntityType::String,
                    is_required: true,
                },
            });
            fields
        },
        parent: None,
    };
    let step = ScenarioStep {
        name: "Преамбула".to_string(),
        header: Some("<p>Комментарий <strong>жирное начертание</strong>, <i>курсив</i>,</p>\n<p><a href=\"https://vk.com\">ссылка</a>.</p>\n".to_string()),
        variables: vec![Var {
            name: "person".to_string(),
            comment: "<p>Введите данные пользователя:</p>".to_string(),
            ty: Entity {
                ty: EntityType::Struct(Struct { name: "Person".to_string(), comment: Some("<p>Комментарий структуры:</p>".to_string()), fields: {
                    let mut fields = IndexMap::new();
                    fields.insert("name".to_string(), Field { comment: "<p>Имя:</p>".to_string(), entity: Entity { ty: EntityType::String, is_required: true } });
                    fields.insert("surname".to_string(), Field { comment: "<p>Фамилия:</p>".to_string(), entity: Entity { ty: EntityType::String, is_required: true } });
                    fields.insert("patronym".to_string(), Field { comment: "<p>Отчество:</p>".to_string(), entity: Entity { ty: EntityType::String, is_required: false } });
                    fields.insert("passport".to_string(), Field { comment: "<p>Паспорт:</p>".to_string(), entity: Entity { ty: EntityType::Struct(passport_struct), is_required: false } });
                    fields
                }, parent: None }),
                is_required: true,
            },
        }],
    };

    Ok(step)
}

#[server(StartOrContinueScenario, "/api", "Url", "start-or-continue-scenario")]
pub async fn start_or_continue_scenario(
    user_id: UserId,
    user_password: UserPassword,
    scenario_id: ScenarioId,
) -> Result<(ScenarioStep, Vec<String>, Option<HashMap<String, dsl::Var>>), ServerFnError> {
    use leptos_axum::redirect;

    let env = Env::from_context()?;

    let is_logged_in = env.login_user(user_id, user_password).await;

    if !is_logged_in {
        return Err(ServerFnError::ServerError(
            "Invalid user id or password".to_string(),
        ));
    }

    let data = env.start_or_continue_scenario(user_id, scenario_id).await;

    data.ok_or_else(|| ServerFnError::ServerError("Could not start scenario".to_string()))
}

#[component]
pub fn ScenarioStep() -> impl IntoView {
    let params = use_params_map();
    let scenario_id: Result<usize, _> = params
        .with(|params| params.get("id").cloned().unwrap())
        .parse();
    let step_id: Option<Result<usize, _>> = params
        .with(|params| params.get("step").cloned())
        .map(|step| step.parse());

    let navigate = use_navigate();
    match (&scenario_id, &step_id) {
        // TODO:
        (Ok(_), Some(Ok(_))) | (Ok(_), None) => {},
        (Ok(scenario_id), Some(Err(_))) => {
            navigate(
                format!("/scenario/{scenario_id}/0").as_str(),
                Default::default(),
            )
        },
        _ => navigate("/", Default::default()),
    }
    let scenario_id = scenario_id.unwrap();
    let step_id = step_id.unwrap_or(Ok(0)).unwrap();

    let step_index: RwSignal<usize> = create_rw_signal(0);
    let current_step = create_resource(step_index, move |_| async { get_scenario_step().await });
    let steps_names = Resource::once(get_steps_names);
    view! {
        <Layout>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=move |_| {
                    view! { <p>"Error happened"</p> }
                }>

                    {move || {
                        match (steps_names(), current_step()) {
                            (Some(Ok(steps_names)), Some(Ok(current_step))) => {
                                let (current_step, _) = create_signal(current_step);
                                let (steps_names, _) = create_signal(steps_names);
                                view! {
                                    <section id="all_page" class="h-full w-full flex flex-row">
                                        <section
                                            id="step"
                                            class="pl-[15px] pr-[15px] pt-[15px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                                        >
                                            <AllSteps steps_names step_index/>
                                        </section>
                                        <StepInput current_step step_index/>
                                        <section class="flex-1 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]">
                                            <div class="w-full h-[45px] border-b-[3px] px-[15px] py-[7px] border-[#8c7456] items-center text-[16px] text-[#8c7456] ">
                                                Предварительный просмотр документа
                                            </div>
                                            <section></section>
                                        </section>
                                    </section>
                                }
                                    .into_view()
                            }
                            (_, _) => view! { <p>"Error happened"</p> }.into_view(),
                        }
                    }}

                </ErrorBoundary>
            </Transition>
        </Layout>
    }
}
