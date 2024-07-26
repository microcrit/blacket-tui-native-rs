use crate::api::types::objects;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericResponse {
    pub error: bool,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub error: bool,
    pub reason: Option<String>,
    pub user: Option<objects::user::User>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataResponse {

}