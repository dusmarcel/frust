use yew::prelude::*;

mod components;

use components::open_file::OpenFile as OpenFile;
use components::file_view::FileView as FileView;

#[function_component]
fn App() -> Html {
    //let fvprops = props!{FileViewProps{}};
    //let on_file_change = Callback::from(move |e: Event| {
    //});
    let file_name = use_state(|| None);

    html! {
        <>
            <h1 class={classes!("text-center")}>{"Willkommen bei Frust!"}</h1>
            <OpenFile file_name={file_name.clone()} />
            <FileView file_name={(*file_name).clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
