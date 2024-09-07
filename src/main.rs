use yew::prelude::*;

mod file;
mod components;

use components::open_archive::OpenArchive as OpenArchive;
use components::archive_view::ArchiveView as ArchiveView;

#[function_component]
fn App() -> Html {
    let archive = use_state(|| None);

    html! {
        <>
            <h1 class={classes!("text-center", "text-2xl", "pt-4")}>{"Willkommen bei Frust!"}</h1>
            <ArchiveView archive={(*archive).clone()} />
            <OpenArchive archive={archive.clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
