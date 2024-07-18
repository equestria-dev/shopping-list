use std::collections::{BTreeMap, HashMap};
use std::fs;

use serde::Deserialize;

use crate::error::WishingStarError;

#[derive(Deserialize, Debug)]
pub struct SteamSubitem {
    pub price: String,
}

#[derive(Deserialize, Debug)]
pub struct SteamData {
    pub name: String,
    pub capsule: String,
    pub release_string: String,
    pub tags: Vec<String>,
    pub subs: Vec<SteamSubitem>,
    pub priority: u32,
}

#[derive(Deserialize)]
pub struct SteamConfig {
    pub enable: bool,
    pub name: String,
    pub id: String,
    pub recommend: u32,
    pub notes: HashMap<u32, String>,
}

#[derive(Deserialize)]
pub struct ItemVariantConfig {
    pub link: String,
    pub name: String,
    pub price: i32,
    pub source: String,
}

#[derive(Deserialize)]
pub struct ItemConfig {
    pub _id: Option<u32>,
    pub name: String,
    pub link: Option<String>,
    #[serde(alias = "links")]
    pub variants: Option<Vec<ItemVariantConfig>>,
    pub price: Option<i32>,
    pub unit: String,
    pub image: String,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub note: Option<String>,
    pub date: Option<String>,
}

pub type IdentifiedListConfig<'a> = (&'a str, &'a ListConfig);
pub type SteamWishlist = HashMap<u32, SteamData>;
pub struct SortedSteamWishlist(BTreeMap<u32, (u32, SteamData)>);

#[derive(Deserialize)]
pub struct ListConfig {
    pub nsfw: bool,
    #[serde(alias = "title")]
    pub title: String,
    pub recommend: u32,
    pub items: Vec<ItemConfig>,
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
    pub lists: HashMap<String, ListConfig>,
}

impl Config {
    pub fn new() -> Result<Self, WishingStarError> {
        let file = fs::read_to_string("./config.yml")?;
        let config: Config = serde_yml::from_str(&file)?;

        Ok(config)
    }
}

impl SortedSteamWishlist {
    pub fn fill_list_config(
        self,
        mut list_config: ListConfig,
        config: &SteamConfig,
        currency: &String,
    ) -> ListConfig {
        for (_, (id, data)) in self.0 {
            let first_sub = &data.subs[0];
            let price_float = first_sub.price.parse::<f32>().unwrap_or(-100.0);
            let price_int = (price_float * 10.0) as i32;

            let item = ItemConfig {
                _id: Some(id),
                name: data.name,
                link: Some(format!("https://store.steampowered.com/app/{id}")),
                variants: None,
                price: Some(price_int),
                unit: format!("% {}", currency),
                image: data.capsule,
                source: Some(String::from("Steam")),
                tags: data.tags,
                note: if let Some(note) = config.notes.get(&id) {
                    Some(note.to_owned())
                } else {
                    None
                },
                date: Some(data.release_string),
            };

            list_config.items.push(item);
        }

        list_config
    }
}

impl ListConfig {
    pub fn get_steam(
        config: &SteamConfig,
        currency: &String,
    ) -> Result<ListConfig, WishingStarError> {
        let steam: SteamWishlist = ureq::get(&format!(
            "https://store.steampowered.com/wishlist/id/{}/wishlistdata/",
            config.id
        ))
        .call()?
        .into_json()?;
        let mut wishlist: SortedSteamWishlist = SortedSteamWishlist(BTreeMap::new());

        for (id, data) in steam {
            wishlist.0.insert(data.priority, (id, data));
        }

        let steam_list = wishlist.fill_list_config(
            ListConfig {
                nsfw: false,
                title: config.name.clone(),
                recommend: config.recommend,
                items: vec![],
            },
            config,
            currency,
        );

        Ok(steam_list)
    }
}
