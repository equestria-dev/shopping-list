use std::collections::BTreeMap;
use std::process::Command;
use std::{fs, process};

use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;

use crate::config::ListConfig;
use crate::config::{Config, IdentifiedListConfig};
use crate::error::WishingStarError;
use crate::html::generate_template;

mod config;
mod error;
mod html;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
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
        let steam_list = ListConfig::get_steam(&config.steam, &config.currency)?;

        info!("Found {} Steam games", steam_list.items.len());
        config.lists.insert(String::from("steam"), steam_list);
    }

    let mut lists_sorted: BTreeMap<u32, IdentifiedListConfig> = BTreeMap::new();
    for (id, list) in &config.lists {
        lists_sorted.insert(!list.recommend, (id, list));
    }

    info!("Rendering HTML");
    fs::write(
        "./app/app.html",
        generate_template(&config, lists_sorted)
            .into_string()
            .as_bytes(),
    )?;

    if config.vercel {
        info!("Deploying to Vercel");

        #[cfg(windows)]
        Command::new("vercel.cmd")
            .current_dir("./app")
            .arg("--prod")
            .status()?;

        #[cfg(not(windows))]
        Command::new("vercel")
            .current_dir("./app")
            .arg("--prod")
            .status()?;
    }

    info!("Done!");

    Ok(())
}
