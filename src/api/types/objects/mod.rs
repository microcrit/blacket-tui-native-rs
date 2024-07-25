use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicClan {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub room: u128
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicMute {
    pub time: i32,
    pub muted: bool,
    pub staff: String,
    pub reason: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicBan {
    pub time: i32,
    pub banned: bool,
    pub staff: String,
    pub reason: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicSettings {
    pub friends: String,
    pub requests: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMisc {
    pub opened: f64,
    pub messages: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,

    pub created: u128,
    pub modified: u128,

    pub avatar: String,
    pub banner: String,
    pub badges: Vec<String>,

    pub blooks: HashMap<String, f64>,
    pub tokens: f64,

    pub perms: Option<Vec<String>>,
    pub clan: Option<PublicClan>,

    pub role: String,
    pub color: String,
    pub exp: f64,
    
    pub inventory: Vec<String>,
    pub mute: Option<PublicMute>,
    pub ban: Option<PublicBan>,

    pub misc: UserMisc,

    pub friends: Vec<i32>,
    pub blocks: Option<Vec<i32>>,
    pub claimed: String,
    
    pub settings: PublicSettings,

    pub otp: Option<bool>,
    pub money_spent: Option<f64>,
}