use dioxus::prelude::*;

const FRUST_CSS: Asset = asset!("/assets/frust.css");

#[component]
pub fn Frust() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FRUST_CSS }

        div { id: "frust",
            h1 { "Frust" }
        }
    }
}