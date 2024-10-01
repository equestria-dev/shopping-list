use maud::{html, Markup};

pub fn budget(currency: &str) -> Markup {
    html! {
        #budget-outer style="margin-bottom: 30px; display: none;" {
            h3 { "Your budget" }

            p style="display: grid; grid-template-columns: 1fr max-content; margin-bottom: 10px;" {
                span {
                    "You allocated "
                    span #budget-value { "0.00" }
                    " " (currency) " and are currently using "
                    span #budget-usage { "0.00" }
                    " " (currency) " ("
                    span #budget-usage-percent { "" } " %)"
                }
                span {
                    b #budget-limit {
                        "0.00 " (currency) " left"
                    }
                }
            }

            div style="display: grid; grid-template-columns: 1fr 1fr;" {
                div style="margin-bottom: 10px;" {
                    input #show-only-selected type="checkbox" onchange="updateBudget()";
                    label for="show-only-selected" { "Only show checked items" }
                }
                div style="margin-bottom: 10px;" {
                    input #hide-oob  type="checkbox" onchange="updateBudget()";
                    label for="hide-oob" { "Hide out of budget items" }
                }
            }
        }
    }
}
