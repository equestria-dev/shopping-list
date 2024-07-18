use std::collections::BTreeMap;

use maud::{html, Markup};

use crate::config::{Config, IdentifiedListConfig};
use crate::html::items::items;
use crate::html::modules::budget::budget;
use crate::html::modules::export::export;
use crate::html::modules::faq::faq;
use crate::html::ui::footer::footer;
use crate::html::ui::heading::heading;

pub fn body(config: &Config, lists: BTreeMap<u32, IdentifiedListConfig>) -> Markup {
    html! {
        body data-bs-theme="light" {
            .container {
                (heading(&config.name))
                (budget(&config.currency))

                @if config.faq {
                    (faq())
                }
                @if config.export {
                    (export())
                }

                @if let Some(notice) = &config.notice {
                    .alert.alert-warning style="margin-bottom: 30px;" { (notice) }
                }

                #filter-results.list-group style="display: none; margin-bottom: 30px;" {}

                (items(lists))
                (footer(&config.currency))
            }
        }
    }
}
