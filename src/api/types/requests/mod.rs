use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
    pub code: Option<String>,
}