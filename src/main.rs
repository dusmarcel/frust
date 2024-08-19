use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{"Willkommen bei Frust!"}</h1>
            <OpenFile />
            <FileView />
        </>
    }
}

#[function_component]
fn OpenFile() -> Html {
    html! {
        <p>{"Open File..."}</p>
    }
}

#[function_component]
fn FileView() -> Html {
    html! {
        <p>{"View File..."}</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
