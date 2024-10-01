use maud::{html, Markup};

use crate::config::ItemConfig;

pub fn base(item: &ItemConfig, price: i32) -> Markup {
    html! {
        div style="display: flex; align-items: center; justify-content: center;" {
            input class="form-check-input item-select" onchange="updateBudget();" type="checkbox" value="" data-price=(price) disabled;
        }

        div {
            div style={
                "background-color: var(--fella-bg-secondary-dim); aspect-ratio: 292/136;"
                "border-radius: 0.375rem; background-repeat: no-repeat; background-size: contain;"
                "background-position: center; background-image: url('"
                (item.image)
                "')"
            } {}
        }
    }
}
