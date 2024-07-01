use maud::{html, Markup};

pub fn heading(name: &str) -> Markup {
    html! {
        br; br;
        h1 { (name) "'s Wishing Star" }

        div style="display: grid; grid-template-columns: 1fr 1fr; grid-gap: 10px;" {
            div {
                select #sort onchange="changeOrder();" class="form-select" {
                    option value="cro" { "Sort by " (name) "'s order" }
                    option value="cnt" { "Sort by " (name) "'s order (no categories)" }
                    option value="plh" { "Sort by price (low to high)" }
                    option value="phl" { "Sort by price (high to low)" }
                    option value="rec" { "Sort by recommendation" }
                }
            }

            input #budget onchange="updateBudget();" type="number" style="height: 38px;" placeholder="Enter your budget here" class="form-control";
        }

        span.small.text-muted style="margin-bottom: 30px; margin-top: 10px; display: inline-block;" {
            "Revert to \"Sort by " (name) "'s order\" to check items and add them to your list. · "
            a #nsfw-toggle onclick="toggleNSFW();" href="#" {
                "Show not safe for work items"
            }
        }
    }
}
