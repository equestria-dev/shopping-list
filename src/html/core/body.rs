use std::collections::BTreeMap;

use maud::{html, Markup};

use crate::config::{Config, IdentifiedListConfig};
use crate::html::items::items;
use crate::html::modules::budget::budget;
use crate::html::modules::export::export;
use crate::html::ui::footer::footer;
use crate::html::ui::heading::heading;
use crate::html::ui::loader::loader;
use crate::html::ui::navigation::navigation;

pub fn body(config: &Config, lists: BTreeMap<u32, IdentifiedListConfig>) -> Markup {
    html! {
        body {
            (loader())

            main #app.fella-escape style="display: none;" {
                (navigation())

                .fella-container.fella-section.fella-section-linked {
                    (heading(&config.name))
                    (budget(&config.currency))

                    @if config.export {
                        (export())
                    }

                    #filter-results.list-group style="display: none; margin-bottom: 30px;" {}

                    (items(lists))
                    (footer(&config.currency))
                }
            }
        }
    }
}
