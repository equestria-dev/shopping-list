use maud::{html, Markup};

use crate::config::ItemConfig;

pub fn price(item: &ItemConfig) -> Markup {
    html! {
        div style="display: flex; align-items: center; justify-content: end;" {
            div style="text-align: right;" {
                @if let Some(price) = item.price {
                    h4 style="text-align: right;" {
                        @if price > 0 {
                            (item.unit.replace("%", &format!("{:.2}", price as f32 / 100.0)))
                        } @else {
                            span.text-muted { (if let Some(date) = &item.date {
                                date
                            } else {
                                "Coming soon"
                            }) }
                        }
                    }
                }
                @if let Some(source) = &item.source {
                    div { (source) }
                }
            }
        }
    }
}
