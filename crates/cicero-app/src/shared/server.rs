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
use std::sync::Arc;

use cicero_dsl::scenario::Scenario;
use tokio::sync::{Mutex, RwLock};

use super::api::*;

// TODO-0:

/// Server environment.
#[derive(Debug, Clone)]
pub struct Env {
    /// Compiled scenarios.
    pub loaded_scenarios: Arc<HashMap<ScenarioId, Scenario>>,
    /// Active scenarios for each user.
    pub active_scenarios: Arc<RwLock<HashMap<UserId, Vec<Scenario>>>>,
    /// Active users (guests).
    pub active_users: Arc<RwLock<HashMap<UserId, UserPassword>>>,
}

impl Env {
    pub fn new(loaded_scenarios: HashMap<ScenarioId, Scenario>) -> Self {
        Self {
            loaded_scenarios: Arc::new(loaded_scenarios),
            active_scenarios: Arc::new(RwLock::new(HashMap::new())),
            active_users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_scenarios(&self, user_id: UserId) -> Option<&[Scenario]> {
        let lock = self.active_scenarios.read().await;
        todo!()
    }

    pub async fn insert(&self, user_id: UserId, scenario: Scenario) {
        self.active_scenarios
            .write()
            .await
            .entry(user_id)
            .or_default()
            .push(scenario);
    }

    pub async fn remove(&self, user_id: UserId, scenario: &Scenario) {
        if let Some(scenarios) = self.active_scenarios.write().await.get_mut(&user_id) {
            scenarios.retain(|s| s.meta() != scenario.meta());
        }
    }
}
