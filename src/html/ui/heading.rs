use maud::{html, Markup};

pub fn heading(name: &str) -> Markup {
    html! {
        h1 .fella-title { (name) "'s Shopping List" }

        div style="margin-bottom: 20px;" {
            input #budget onchange="updateBudget();" type="number" placeholder="Enter your budget here" class="fella-form-control";
        }
    }
}
