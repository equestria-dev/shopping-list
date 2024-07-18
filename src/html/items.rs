use std::collections::BTreeMap;

use maud::{html, Markup};

use crate::config::IdentifiedListConfig;
use crate::html::items::list::list;

mod base;
mod details;
mod item;
mod link;
mod list;
mod price;
mod variants;

pub fn items(lists: BTreeMap<u32, IdentifiedListConfig>) -> Markup {
    html! {
        #all-items {
            @for (_, (id, config)) in &lists {
                (list(id, config));
            }
        }
    }
}
