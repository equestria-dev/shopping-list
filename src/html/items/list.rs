use maud::{html, Markup};

use crate::config::ListConfig;
use crate::html::items::item::item;

pub fn list(id: &str, list: &ListConfig) -> Markup {
    html! {
        div class=(if list.nsfw {
            "list-container list-container-nsfw"
        } else {
            "list-container"
        }) {
            h3 { (list.title) }
            .list-group id={ "list-" (id) } style="margin-bottom: 30px;" {
                @for list_item in &list.items {
                    (item(&list_item, &list))
                }
            }
        }
    }
}
