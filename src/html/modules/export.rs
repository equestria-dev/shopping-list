use maud::{html, Markup};

pub fn export() -> Markup {
    html! {
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
}
