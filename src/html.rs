use maud::{DOCTYPE, html, Markup, PreEscaped};

use crate::config::Config;

pub fn generate_template(config: Config) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="ie=edge";

                title { (config.name) "'s Wishing Star" }

                link href="/assets/bootstrap.min.css" rel="stylesheet";
                link href="/assets/logo.png" rel="shortcut icon" type="image/png";
                script src="/assets/bootstrap.bundle.min.js" {}

                style {
                    (PreEscaped(r#"
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
                    "#))
                }
            }

            body data-bs-theme="light" {
                .container {
                    br; br;
                    h1 { (config.name) "'s Wishing Star" }

                    div style="display: grid; grid-template-columns: 1fr 1fr; grid-gap: 10px;" {
                        div {
                            select #sort onchange="changeOrder();" class="form-select" {
                                option value="cro" { "Sort by " (config.name) "'s order" }
                                option value="cnt" { "Sort by " (config.name) "'s order (no categories)" }
                                option value="plh" { "Sort by price (low to high)" }
                                option value="phl" { "Sort by price (high to low)" }
                                option value="alh" { "Sort by Predicted appreciation (low to high)" }
                                option value="ahl" { "Sort by Predicted appreciation (high to low)" }
                                option value="rec" { "Sort by recommendation" }
                            }
                        }

                        input #budget onchange="updateBudget();" type="number" style="height: 38px;" placeholder="Enter your budget here" class="form-control";
                    }

                    span.small.text-muted style="margin-bottom: 30px; margin-top: 10px; display: inline-block;" {
                        "Revert to \"Sort by " (config.name) "'s order\" to check items and add them to your list. · "
                        a #nsfw-toggle onclick="toggleNSFW();" href="#" {
                            "Show not safe for work items"
                        }
                    }

                    #budget-outer style="margin-bottom: 30px; display: none;" {
                        h3 { "Your budget" }

                        p style="display: grid; grid-template-columns: 1fr max-content; margin-bottom: 10px;" {
                            span {
                                "You allocated"
                                span #budget-value { "0.00" }
                                " " (config.currency) " and are currently using"
                                span #budget-usage { "0.00" }
                                " " (config.currency) " ("
                                span #budget-usage-percent { "" } " %)"
                            }
                            span {
                                b #budget-limit {
                                    "0.00 " (config.currency) " left"
                                }
                            }
                        }

                        .progress style="margin-bottom: 10px;" {
                            #budget-progress.progress-bar {}
                            #budget-progress-red.progress-bar.bg-danger {}
                        }

                        div style="display: grid; grid-template-columns: 1fr 1fr;" {
                            .form-check style="margin-bottom: 10px;" {
                                input #hide-oob class="form-check-input" type="checkbox" onchange="updateBudget()";
                                label class="form-check-label" for="hide-oob" { "Hide out of budget items" }
                            }
                        }
                    }

                    @if config.faq {
                        h3 { "Frequently asked questions" }
                        #faq.accordion style="margin-bottom: 15px;" {
                            .accordion-item {
                                h2.accordion-header {
                                    button.accordion-button.collapsed type="button" data-bs-toggle="collapse" data-bs-target="#faq-1" {
                                        "What is the \"Predicted appreciation\"?"
                                    }
                                }

                                #"faq-1".accordion-collapse.collapse data-bs-parent="#faq" {
                                    .accordion-body {
                                        r#"For some items, such as Steam games, a score is calculated based on user ratings and compatibility
                                        with the hardware we own (in the case of Steam games) or similarity with other products we own
                                        (in other cases). Therefore, this score indicates how likely we are to like a specific product if
                                        we do acquire it."#
                                    }
                                }
                            }
                            .accordion-item {
                                h2.accordion-header {
                                    button.accordion-button.collapsed type="button" data-bs-toggle="collapse" data-bs-target="#faq-2" {
                                        "What do \"Most Wanted\" and \"Recommended\" mean?"
                                    }
                                }

                                #"faq-2".accordion-collapse.collapse data-bs-parent="#faq" {
                                    .accordion-body {
                                        p {
                                            r#"The "Most Wanted" badge shows for an item that's at the top of our list when sorted by
                                            our own priority. It is especially useful to see what we want the most and base your purchase
                                            decision on this if you have no idea."#
                                        }
                                        div {
                                            r#"The "Recommended" badge uses an algorithm to recommend the best items to buy according to
                                            your budget. It is a great way to know what to give us if you are on a very low budget or you
                                            don't know what to buy. Also check out the "Sort by recommendation" option."#
                                        }
                                    }
                                }
                            }
                            .accordion-item {
                                h2.accordion-header {
                                    button.accordion-button.collapsed type="button" data-bs-toggle="collapse" data-bs-target="#faq-3" {
                                        "Can I get items in other places?"
                                    }
                                }

                                #"faq-3".accordion-collapse.collapse data-bs-parent="#faq" {
                                    .accordion-body {
                                        p {
                                            r#"For Steam games, they must be purchased from Steam (so that they can work out-of-the-box
                                            on Steam Deck)."#
                                        }
                                        div {
                                            r#"For other items, the same item can be purchased from another place without asking us, and
                                            a different (but similar, e.g. cheaper) item can be purchased from another place after asking us."#
                                        }
                                    }
                                }
                            }
                            .accordion-item {
                                h2.accordion-header {
                                    button.accordion-button.collapsed type="button" data-bs-toggle="collapse" data-bs-target="#faq-4" {
                                        "How should I pay for these items?"
                                    }
                                }

                                #"faq-4".accordion-collapse.collapse data-bs-parent="#faq" {
                                    .accordion-body {
                                        p {
                                            r#"For items that are available on Steam, Amazon, Google Play, Apple stores and other
                                            compatible platforms, a gift card would be preferred to prevent issues with delivery.
                                            Steam games must be purchased with gift cards due to regional restrictions. For items
                                            that cannot be bought with a gift card, you will have to order them yourself and enter
                                            our address."#
                                        }
                                        div {
                                            r#"You can also order items yourself and ship them to yourself, then send them to us.
                                            That way, you can bundle multiple items together and/or add personal items or notes."#
                                        }
                                    }
                                }
                            }
                        }
                    }

                    @if config.export {
                        .small.text-muted style="margin-bottom: 15px;" {
                            r#"All the data associated with your settings and budget on this page is saved on this browser only and is
                            not sent to Equestria.dev or to your other devices. If you would like to transfer this data to other devices
                            or back it up, you will need to "#
                            a href="#" onclick="dataImport();" { "import" }
                            " or "
                            a href="#" onclick="dataExport();" { "export" }
                            " it."
                        }
                    }

                    @if let Some(notice) = config.notice {
                        .alert.alert-warning style="margin-bottom: 30px;" { (notice) }
                    }

                    #filter-results.list-group style="display: none; margin-bottom: 30px;" {}

                    #all-items {
                        // TODO
                    }

                    br; br;

                    script {
                        (PreEscaped("window.currency = \"")) (config.currency) (PreEscaped("\";"))
                    }

                    script src="/assets/app.js" {}
                }
            }
        }
    }
}
