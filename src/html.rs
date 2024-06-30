use maud::{DOCTYPE, html, Markup};
use crate::config::Config;

pub fn generate_template(config: Config) -> Markup {
    html! {
        (DOCTYPE)
    }
}
