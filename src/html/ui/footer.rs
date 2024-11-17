use maud::{html, Markup, PreEscaped};

pub fn footer(currency: &str) -> Markup {
    html! {
        br; br;

        script {
            (PreEscaped("window.currency = \"")) (currency) (PreEscaped("\";"))
        }

        script src="/assets/app.js" {}
        script src="https://fella.floo.fi/fella.min.js" {}
    }
}
