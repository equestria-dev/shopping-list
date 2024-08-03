use maud::{html, Markup};

pub fn faq() -> Markup {
    html! {
        h3 { "Frequently asked questions" }
        #faq.accordion style="margin-bottom: 15px;" {
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
                            r#"For items that are available on Steam, Amazon, Google Play and other
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
}
