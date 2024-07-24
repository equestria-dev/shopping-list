use maud::{html, Markup};

use crate::config::{ItemConfig, ListConfig};

pub fn link(inner: Markup, list: &ListConfig, item: &ItemConfig, price: i32, id: &str) -> Markup {
    html! {
        @if let Some(link) = &item.link {
            a data-recommend=(list.recommend) data-list=(list.title)
                id={ "item-" (list.title) "-" (id) } data-score="0"
                data-price=(price)
                href=(link)
                class={
                    "list-item list-item-sel list-group-item "
                    (
                        if let Some(_) = &item.link {
                            "list-group-item-action "
                        } else {
                            " "
                        }
                    )
                    (
                        if list.nsfw {
                            "list-item-nsfw"
                        } else {
                            ""
                        }
                    )
                } {
                (inner)
            }
        } else {
            div data-recommend=(list.recommend) data-list=(list.title)
                id={ "item-" (list.title) "-" (id) } data-score="0"
                data-price=(price)
                class={
                    "list-item list-item-sel list-group-item "
                    (
                        if let Some(_) = &item.link {
                            "list-group-item-action "
                        } else {
                            " "
                        }
                    )
                    (
                        if list.nsfw {
                            "list-item-nsfw"
                        } else {
                            ""
                        }
                    )
                } {
                (inner)
            }
        }
    }
}
