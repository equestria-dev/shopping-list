use maud::{html, Markup, PreEscaped};

const STYLESHEET: &str = r#"
    body:not(.show-nsfw) .list-item-nsfw, body:not(.show-nsfw) .list-container-nsfw {
        display: none;
    }

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

    .custom-badge {
        background-color: rgba(var(--custom-badge-base, (0, 0, 0)), 1);
    }

    @media (prefers-color-scheme: dark) {
        .custom-badge {
            background-color: rgba(var(--custom-badge-base, (0, 0, 0)), .5);
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

            title { (name) "'s Wishing Star" }

            link href="/assets/bootstrap.min.css" rel="stylesheet";
            link href="/assets/logo.png" rel="shortcut icon" type="image/png";
            script src="/assets/bootstrap.bundle.min.js" {}

            style { (PreEscaped(STYLESHEET)) }
        }
    }
}
