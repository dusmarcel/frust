use leptos::prelude::*;

mod header;
use crate::header::Header;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
    }
}