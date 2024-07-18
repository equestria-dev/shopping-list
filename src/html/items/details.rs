use maud::{html, Markup};

use crate::config::ItemConfig;
use crate::html::items::variants::variants;

pub fn details(item: &ItemConfig) -> Markup {
    html! {
        div {
            @if let Some(note) = &item.note {
                .text-muted {
                    i { (note) }
                }
            }
            @if item.tags.len() > 0 {
                div { (item.tags.join(" · ")) }
            }
            @if let Some(variants_list) = &item.variants {
                (variants(variants_list, &item.unit))
            }
        }
    }
}
