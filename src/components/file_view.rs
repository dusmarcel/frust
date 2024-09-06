use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    // ....
}

#[function_component(FileView)]
pub fn file_view(props: &FileViewProps) -> Html {
    html! {
        <p>
            {"Hallo, Grid!"}
        </p>
    }
}
