use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicItemSale {
    pub price: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicStoreItem {
    pub price: String,
    pub sale: Option<PublicItemSale>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicExp {
    pub difficulty: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicPage {
    pub link: String,
    pub icon: String,

    pub is_news: bool,
    pub is_chat: bool,

    pub location: String,
    pub perm: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicChat {
    pub tokens: i32,
    pub exp: i32,
    pub cooldown: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicReportTypes {
    pub user: HashMap<String, Vec<String>>,
    pub message: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicBooster {
    pub active: bool,
    pub multiplier: i32,
    pub time: f64,
    pub user: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicCreditsUser {
    pub user: i32,
    pub nickname: String,
    pub image: Option<String>,
    pub top: Option<bool>,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicNewsPost {
    pub title: String,
    pub image: String,
    pub body: String,
    pub date: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicPreset {
    pub color: String,
    pub perms: Vec<String>,
    pub badges: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicBlook {
    pub rarity: String,

    pub chance: i32,
    pub price: i32,

    pub image: String,
    pub art: String,

    pub only_on_day: Option<bool>,

    pub bazaar_minimum_listing_price: Option<i32>,
    pub bazaar_maximum_listing_price: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicRarity {
    pub color: String,
    pub animation: String,
    pub exp: i32,
    pub wait: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicPack {
    pub price: i32,
    
    pub color1: String,
    pub color2: String,

    pub image: String,

    pub blooks: Vec<String>,

    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicImageful {
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicBadge {
    pub image: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicWeeklyShopItem {
    pub price: i32,
    pub glow: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlacketConfig {
    pub name: String,
    pub version: String,
    
    pub welcome: String,
    pub description: String,
    pub pronunciation: String,
    pub discord: String,

    pub store: HashMap<String, PublicStoreItem>,
    pub rewards: Vec<i32>,
    pub exp: PublicExp,

    pub pages: HashMap<String, PublicPage>,

    pub chat: PublicChat,
    pub reports: PublicReportTypes,

    pub booster: PublicBooster,

    pub credits: Vec<PublicCreditsUser>,

    pub news: Vec<PublicNewsPost>,
    
    pub presets: HashMap<String, PublicPreset>,

    pub blooks: HashMap<String, PublicBlook>,
    pub rarities: HashMap<String, PublicRarity>,
    pub packs: HashMap<String, PublicPack>,

    pub banners: HashMap<String, PublicImageful>,
    pub badges: HashMap<String, PublicBadge>,

    pub emojis: HashMap<String, PublicImageful>,

    #[serde(rename = "weekly_shop")]
    pub weekly_shop: HashMap<String, PublicWeeklyShopItem>,
}