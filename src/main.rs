use yew::prelude::*;

mod components;

use components::open_file::OpenFile as OpenFile;
use components::file_view::FileView as FileView;

#[function_component]
fn App() -> Html {
    let file = use_state(|| None);

    html! {
        <>
            <h1 class={classes!("text-center", "text-2xl", "pt-4")}>{"Willkommen bei Frust!"}</h1>
            <FileView file={(*file).clone()} />
            <OpenFile file={file.clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
