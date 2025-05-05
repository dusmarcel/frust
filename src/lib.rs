use leptos::prelude::*;

mod header;
use crate::header::Header;

mod openfile;
use crate::openfile::OpenFile;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <OpenFile />
    }
}