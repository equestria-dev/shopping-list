use maud::{html, Markup, PreEscaped};

const STYLESHEET: &str = r#"
    .list-item {
        display: grid;
        grid-template-columns: 8px 1fr 3fr 150px;
        grid-gap: 20px;
        border-radius: 0 !important;
    }

    @media (max-width: 700px) {
        .list-item {
            grid-template-columns: 1fr !important;
        }
    }

    #filter-results input {
        display: none;
    }
"#;

pub fn head(name: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0";
            meta http-equiv="X-UA-Compatible" content="ie=edge";

            title { (name) "'s Shopping List" }

            link href="https://fella.floo.fi/0.3.3/fella.min.css" rel="stylesheet"
                integrity="sha384-1Im8AmCEPI7+gL0sQFNekjJ/RE5oo3q2Up8muPiUO5Ho6QKmdx74KgECaEx4yWqV"
                crossorigin="anonymous";
            link href="/assets/logo.png" rel="shortcut icon" type="image/png";

            style { (PreEscaped(STYLESHEET)) }
        }
    }
}
