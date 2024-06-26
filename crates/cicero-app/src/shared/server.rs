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

use core::panic;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use cicero_dsl::data as dsl;
use cicero_dsl::scenario::Scenario;
use cicero_dsl::types::{ScenarioMeta, ScenarioStep};
use indexmap::IndexMap;
use leptos::server_fn::error::NoCustomError;
use leptos::*;
use rand::Rng;
use tokio::process::Command;
use tokio::sync::{Mutex, RwLock};

use super::api::*;

const PASSWORD_LENGTH: usize = 64;

// TODO-0:

/// Server environment.
#[derive(Debug, Clone)]
pub struct Env {
    /// Compiled scenarios.
    pub loaded_scenarios: Arc<HashMap<ScenarioId, Scenario>>,
    pub scenarios_metas: Arc<IndexMap<String, Vec<ScenarioMeta>>>,
    /// Active scenarios for each user.
    pub active_scenarios: Arc<RwLock<HashMap<UserId, Vec<Scenario>>>>,
    /// Active users (guests).
    pub active_users: Arc<RwLock<HashMap<UserId, UserPassword>>>,
    pub last_user_id: Arc<AtomicU64>,
}

impl Env {
    pub fn new(loaded_scenarios: HashMap<ScenarioId, Scenario>) -> Self {
        let mut metas: IndexMap<String, Vec<ScenarioMeta>> = IndexMap::new();

        for scenario in loaded_scenarios.values() {
            let meta = scenario.meta();
            metas
                .entry(meta.category.clone())
                .or_default()
                .push(meta.clone());
        }

        Self {
            loaded_scenarios: Arc::new(loaded_scenarios),
            scenarios_metas: Arc::new(metas),
            active_scenarios: Arc::new(RwLock::new(HashMap::new())),
            active_users: Arc::new(RwLock::new(HashMap::new())),
            last_user_id: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Infer environment from the server context.
    pub fn from_context() -> Result<Self, ServerFnError> {
        use_context::<Env>().ok_or_else(|| ServerFnError::ServerError("Env is missing".to_string()))
    }

    pub async fn start_or_continue_scenario(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
        desired_step_id: usize,
    ) -> Option<(
        ScenarioStep,
        usize,
        Vec<String>,
        Option<HashMap<String, dsl::Var>>,
    )> {
        let running_scenario = self
            .get_running_scenario(user_id, scenario_id, desired_step_id)
            .await;

        match running_scenario {
            Some(data) => Some(data),
            None => {
                self.start_scenario(user_id, scenario_id)
                    .await
                    .map(|(step, steps_names)| (step, 0, steps_names, None))
            },
        }
    }

    async fn get_running_scenario(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
        desired_step_id: usize,
    ) -> Option<(
        ScenarioStep,
        usize,
        Vec<String>,
        Option<HashMap<String, dsl::Var>>,
    )> {
        let mut lock = self.active_scenarios.write().await;

        let running_scenario = lock.get_mut(&user_id).and_then(|scenarios| {
            scenarios
                .iter_mut()
                .find(|scenario| scenario.meta().id == scenario_id)
        });

        match running_scenario {
            Some(scenario) => {
                if scenario.step_to(desired_step_id).is_err() {
                    scenario
                        .step_to(scenario.pending_step())
                        .expect("Pending step should be valid");
                };

                let step = scenario.scenario_step();
                let pending_step_id = scenario.pending_step();
                let steps_names = scenario.steps_names();
                let data = scenario.current_step_data();

                Some((step, pending_step_id, steps_names, data))
            },
            None => None,
        }
    }

    async fn start_scenario(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
    ) -> Option<(ScenarioStep, Vec<String>)> {
        let scenario = self.loaded_scenarios.get(&scenario_id).cloned();

        match scenario {
            Some(scenario) => {
                self.insert(user_id, scenario.clone()).await;

                let step = scenario.scenario_step();
                let steps_names = scenario.steps_names();

                Some((step, steps_names))
            },
            None => None,
        }
    }

    async fn insert(&self, user_id: UserId, scenario: Scenario) {
        let mut lock = self.active_scenarios.write().await;
        let running_scenarios = lock.entry(user_id).or_default();

        let replace = running_scenarios
            .iter_mut()
            .find(|scenario| scenario.meta().id == scenario.meta().id);

        match replace {
            Some(running_scenario) => {
                *running_scenario = scenario;
            },
            None => {
                running_scenarios.push(scenario);
            },
        }
    }

    async fn remove(&self, user_id: UserId, scenario: &Scenario) {
        if let Some(scenarios) = self.active_scenarios.write().await.get_mut(&user_id) {
            scenarios.retain(|s| s.meta() != scenario.meta());
        }
    }

    async fn get_user(&self, user_id: UserId) -> Option<UserPassword> {
        self.active_users.read().await.get(&user_id).cloned()
    }

    pub async fn login_user(&self, user_id: UserId, user_password: UserPassword) -> bool {
        // NB: as all users are guests and passwords are auto-generated, this is safe
        self.get_user(user_id)
            .await
            .map_or(false, |password| password == user_password)
    }

    pub async fn register_user(&self) -> (UserId, UserPassword) {
        use rand::prelude::*;

        let user_id = self.last_user_id.fetch_add(1, Ordering::Relaxed);
        let random_string: String = rand::thread_rng()
            .sample_iter::<char, _>(rand::distributions::Standard)
            .take(64)
            .collect();

        let is_replaced = self
            .active_users
            .write()
            .await
            .insert(user_id, random_string.clone())
            .is_some();

        if is_replaced {
            panic!("User ID collision");
        }

        (user_id, random_string)
    }

    pub async fn render_scenario_step(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
        step_id: usize,
    ) -> Result<Vec<String>, ServerFnError> {
        let lock = self.active_scenarios.read().await;

        let scenario = lock
            .get(&user_id)
            .ok_or_else(|| {
                ServerFnError::<NoCustomError>::ServerError("User not found".to_string())
            })?
            .iter()
            .find(|scenario| scenario.meta().id == scenario_id)
            .ok_or_else(|| {
                ServerFnError::<NoCustomError>::ServerError("Scenario not found".to_string())
            })?
            .clone();

        let mut scenario_data_path = PathBuf::from("rendered/data");
        scenario_data_path.push(format!("{user_id}"));
        scenario_data_path.push(format!("{scenario_id}"));
        tokio::fs::create_dir_all(&scenario_data_path)
            .await
            .unwrap();

        let mut data_path = scenario_data_path.clone();
        data_path.push(format!("{step_id}"));
        tokio::fs::create_dir_all(&data_path).await.unwrap();
        tokio::fs::remove_dir_all(&data_path).await.unwrap();
        tokio::fs::create_dir_all(&data_path).await.unwrap();

        // create ascii random string for the pages
        let random_string: String = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .map(char::from)
            .take(32)
            .collect();
        data_path.push(format!("{}-page", &random_string));

        let rendered_pdf_path =
            tokio::task::spawn_blocking(move || scenario.render_pdf(scenario_data_path))
                .await
                .unwrap()
                .map_err(|e| {
                    ServerFnError::<NoCustomError>::ServerError(format!(
                        "Failed to render PDF: {e}"
                    ))
                })?;

        let mut command = Command::new("pdftoppm");
        command.args([
            OsStr::new("-jpeg"),
            rendered_pdf_path.as_os_str(),
            data_path.as_ref(),
        ]);
        let exit_status = command
            .spawn()
            .expect("Malformed data")
            .wait()
            .await
            .expect("Malformed data");

        if !exit_status.success() {
            return Err(ServerFnError::<NoCustomError>::ServerError(
                "Failed to convert PDF to images".to_string(),
            ));
        }

        // Remove "page" file prefix
        data_path.pop();

        let mut images = Vec::new();
        for i in 1.. {
            let mut image_path = data_path.clone();
            image_path.push(format!("{}-page-{i}.jpg", &random_string));

            if !image_path.exists() {
                break;
            }

            let image_path = image_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            images.push(image_path);
        }

        Ok(images)
    }

    pub async fn full_render_pdf(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
    ) -> Option<PathBuf> {
        let lock = self.active_scenarios.read().await;

        let scenario = lock
            .get(&user_id)?
            .iter()
            .find(|scenario| scenario.meta().id == scenario_id)?
            .clone();

        let mut scenario_data_path = PathBuf::from("data");
        scenario_data_path.push(format!("{user_id}"));
        scenario_data_path.push(format!("{scenario_id}"));
        tokio::fs::create_dir_all(&scenario_data_path)
            .await
            .unwrap();

        tokio::task::spawn_blocking(move || scenario.full_render_pdf(scenario_data_path))
            .await
            .unwrap()
            .ok()
    }

    pub async fn full_render_docx(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
    ) -> Option<PathBuf> {
        let lock = self.active_scenarios.read().await;

        let scenario = lock
            .get(&user_id)?
            .iter()
            .find(|scenario| scenario.meta().id == scenario_id)?
            .clone();

        let mut scenario_data_path = PathBuf::from("data");
        scenario_data_path.push(format!("{user_id}"));
        scenario_data_path.push(format!("{scenario_id}"));
        tokio::fs::create_dir_all(&scenario_data_path)
            .await
            .unwrap();

        let mut docx_path = scenario_data_path.clone();
        docx_path.push("rendered.docx");

        let mut scenario_reference_path = PathBuf::from("scenarios");
        scenario_reference_path.push(format!("{scenario_id}"));
        scenario_reference_path.push("reference.docx");

        tokio::task::spawn_blocking(move || {
            scenario.full_render_docx(scenario_data_path, scenario_reference_path)
        })
        .await
        .unwrap()
        .ok()
        .map(|_| docx_path)
    }

    pub async fn insert_data(
        &self,
        user_id: UserId,
        scenario_id: ScenarioId,
        step_id: usize,
        data: HashMap<String, dsl::Var>,
    ) -> Result<usize, ServerFnError> {
        let mut lock = self.active_scenarios.write().await;

        let scenario = lock
            .get_mut(&user_id)
            .ok_or(ServerFnError::<NoCustomError>::ServerError(
                "User not found".to_string(),
            ))?
            .iter_mut()
            .find(|scenario| scenario.meta().id == scenario_id)
            .ok_or(ServerFnError::<NoCustomError>::ServerError(
                "Scenario not found".to_string(),
            ))?;

        scenario
            .step_to(step_id)
            .map_err(|err| ServerFnError::<NoCustomError>::ServerError(err.to_string()))?;
        scenario
            .insert_data(data)
            .map_err(|err| ServerFnError::<NoCustomError>::ServerError(err.to_string()))?;

        Ok(scenario.pending_step())
    }
}
