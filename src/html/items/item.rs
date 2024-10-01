use maud::{html, Markup};

use crate::config::{ItemConfig, ListConfig};
use crate::html::items::base::base;
use crate::html::items::details::details;
use crate::html::items::link::link;
use crate::html::items::price::price;

pub fn item(item: &ItemConfig, list: &ListConfig) -> Markup {
    html! {
        @let id = if let Some(id) = item._id {
            id.to_string()
        } else {
            format!("{:x}", md5::compute(item.name.as_bytes()))
        };

        @let item_price = if let Some(variants) = &item.variants {
            let prices: Vec<i32> = variants.iter()
                .map(|i| i.price)
                .collect();
            prices.iter().sum::<i32>() / prices.len() as i32
        } else if let Some(price) = item.price {
            price
        } else {
            -200
        };

        (link(html! {
            (base(item, item_price))

            div style="display: flex; align-items: center; margin-top: 10px; margin-bottom: 10px;" {
                div {
                    div {
                        span.fella-badge-notice.badge-budget id={ "badge-budget-item-" (list.title) "-" (id) } style="margin-bottom: 5px; display: none; --fella-badge-notice-rgb: 32, 201, 151 !important;" {
                            "Recommended"
                        }
                        span.fella-badge-notice.badge-most style="display: none; margin-bottom: 5px; --fella-badge-notice-rgb: 111, 66, 193 !important;" {
                            "Most Wanted"
                        }
                    }
                    h3 style="margin-top: 0; margin-bottom: 5px;" { (item.name) }
                    (details(item))
                }
            }
            (price(item))
        }, list, item, item_price, &id))
    }
}
