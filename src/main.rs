mod config;
mod error;
mod html;

use std::{fs, process};
use std::collections::HashMap;
use config::{ItemConfig, ListConfig, SteamSubitem};
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use crate::config::{Config, SteamData};
use crate::error::WishingStarError;
use crate::html::generate_template;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    if let Err(e) = start() {
        error!("An error has occurred: {e:?}");
        process::exit(1);
    }
}

fn start() -> Result<(), WishingStarError> {
    info!("Reading configuration...");

    let mut config = Config::new()?;

    if config.steam.enable {
        info!("Gathering Steam data...");

        let steam: HashMap<u32, SteamData> = ureq::get(&format!("https://store.steampowered.com/wishlist/id/{}/wishlistdata/", config.steam.id))
            .call()?
            .into_json()?;

        let mut steam_list: ListConfig = ListConfig {
            nsfw: false,
            title: config.steam.name.clone(),
            recommend: config.steam.recommend,
            items: vec![],
        };

        for (id, data) in steam {
            let first_sub = &data.subs[0];
            let price_float = first_sub.price.parse::<f32>().unwrap_or(-100.0);
            let price_int = (price_float * 10.0) as i32;

            let item = ItemConfig {
                _id: Some(id),
                name: data.name,
                link: Some(format!("https://store.steampowered.com/app/{id}")),
                links: None,
                price: Some(price_int),
                unit: format!("% {}", config.currency),
                image: data.capsule,
                source: Some(String::from("Steam")),
                tags: data.tags,
                note: None,
                date: Some(data.release_string),
            };

            steam_list.items.push(item);
        }

        info!("Found {} Steam games", steam_list.items.len());
        config.lists.insert(String::from("steam"), steam_list);
    }

    info!("Rendering HTML");
    fs::write("./app/app.html", generate_template(config).into_string().as_bytes())?;

    Ok(())
}
