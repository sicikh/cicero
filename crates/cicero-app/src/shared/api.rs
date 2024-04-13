use serde::{Deserialize, Serialize};

pub type UserId = u64;
pub type UserPassword = String;
pub type ScenarioId = u64;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub password: UserPassword,
}
