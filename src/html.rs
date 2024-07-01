use std::collections::BTreeMap;

use maud::{html, Markup, DOCTYPE};

use crate::config::{Config, IdentifiedListConfig};
use crate::html::core::body::body;
use crate::html::core::head::head;

mod core;
mod items;
mod modules;
mod ui;

pub fn generate_template(config: &Config, lists: BTreeMap<u32, IdentifiedListConfig>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head(&config.name))
            (body(config, lists))
        }
    }
}
