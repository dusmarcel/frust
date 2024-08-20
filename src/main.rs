use yew::{prelude::*, props};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    filename: Option<String>,
}

#[function_component]
fn App() -> Html {
    let props = props! (Props {
        filename: None::<String>,
    });

    html! {
        <>
            <h1>{"Willkommen bei Frust!"}</h1>
            <OpenFile ..props.clone() />
            <FileView ..props />
        </>
    }
}

#[function_component]
fn OpenFile(props: &Props) -> Html {
    html! {
        <input type="file" accept="application/zip" />
    }
}

#[function_component]
fn FileView(props: &Props) -> Html {
    html! {
        <p>{"View File..."}</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
