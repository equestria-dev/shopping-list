use maud::{html, Markup};

use crate::config::ItemVariantConfig;

pub fn variants(variants_list: &Vec<ItemVariantConfig>, unit: &str) -> Markup {
    html! {
        .fella-list-container style="margin-top: 10px;" {
            @for variant in variants_list {
                .fella-list-item.fella-list-item-padded.fella-list-link style="cursor: pointer;" onclick={ "window.open(`" (variant.link) "`);" } {
                    div style="display: grid; grid-template-columns: 1fr max-content;" {
                        div { (variant.name) }
                        div {
                            b { (unit.replace("%", &format!("{:.2}", variant.price as f32 / 100.0))) }
                            " · "
                            span { (variant.source) " ↗" }
                        }
                    }
                }
            }
        }
    }
}
