use yew::{prelude::*, props};

#[derive(Properties, PartialEq)]
pub struct OpenFileProps {
    pub on_file_change: Callback<Event>,
}

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    #[prop_or(AttrValue::from("No file selected"))]
    pub msg: AttrValue,
}

#[function_component]
fn App() -> Html {
    let fvprops = props!{FileViewProps{}};
    let on_file_change = Callback::from(move |e: Event| {
    });

    html! {
        <>
            <h1 class={classes!("text-center")}>{"Willkommen bei Frust!"}</h1>
            <OpenFile {on_file_change} />
            <FileView ..fvprops />
        </>
    }
}

#[function_component]
fn OpenFile(props: &OpenFileProps) -> Html {
    let onchange = props.on_file_change.clone();

    html! {
        <input
            id="file-upload"
            type="file"
            accept="application/zip"
            {onchange}
        />
    }
}

#[function_component]
fn FileView(props: &FileViewProps) -> Html {
    let msg = props.msg.clone();

    html! {
        <p>{msg}</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
