use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use crate::error::WishingStarError;

#[derive(Deserialize, Debug)]
pub struct SteamSubitem {
    pub price: String
}

#[derive(Deserialize, Debug)]
pub struct SteamData {
    pub name: String,
    pub capsule: String,
    pub release_string: String,
    pub tags: Vec<String>,
    pub subs: Vec<SteamSubitem>
}

#[derive(Deserialize)]
pub struct SteamConfig {
    pub enable: bool,
    pub name: String,
    pub id: String,
    pub recommend: u32,
    pub notes: HashMap<u32, String>
}

#[derive(Deserialize)]
pub struct ItemLinkConfig {
    pub link: String,
    pub name: String,
    pub price: u32,
    pub source: String
}

#[derive(Deserialize)]
pub struct ItemConfig {
    pub _id: Option<u32>,
    pub name: String,
    pub link: Option<String>,
    pub links: Option<Vec<ItemLinkConfig>>,
    pub price: Option<i32>,
    pub unit: String,
    pub image: String,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub note: Option<String>,
    pub date: Option<String>
}

#[derive(Deserialize)]
pub struct ListConfig {
    pub nsfw: bool,
    #[serde(alias = "title")]
    pub title: String,
    pub recommend: u32,
    pub items: Vec<ItemConfig>
}

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub vercel: bool,
    pub currency: String,
    pub faq: bool,
    pub export: bool,
    pub notice: Option<String>,
    pub steam: SteamConfig,
    #[serde(rename = "custom")]
    pub lists: HashMap<String, ListConfig>
}

impl Config {
    pub fn new() -> Result<Self, WishingStarError> {
        let file = fs::read_to_string("./config.yml")?;
        let config: Config = serde_yml::from_str(&file)?;
        Ok(config)
    }
}
