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

            .progress style="margin-bottom: 10px;" {
                #budget-progress.progress-bar {}
                #budget-progress-red.progress-bar.bg-danger {}
            }

            div style="display: grid; grid-template-columns: 1fr 1fr;" {
                .form-check style="margin-bottom: 10px;" {
                    input #show-only-selected class="form-check-input" type="checkbox" onchange="updateBudget()";
                    label class="form-check-label" for="show-only-selected" { "Only show checked items" }
                }
                .form-check style="margin-bottom: 10px;" {
                    input #hide-oob class="form-check-input" type="checkbox" onchange="updateBudget()";
                    label class="form-check-label" for="hide-oob" { "Hide out of budget items" }
                }
            }
        }
    }
}
