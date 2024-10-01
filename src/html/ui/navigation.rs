use maud::{html, Markup};

pub fn navigation() -> Markup {
    html! {
        nav #navbar.fella-nav-no-border.fella-nav {
            .fella-nav-inner {
                .fella-nav-left {
                    a href="/" target="_blank" {
                        img .fella-nav-icon alt="Floofi Shopping List" src="/assets/wordmark.png";
                    }
                }
            }
        }
    }
}
