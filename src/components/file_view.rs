use yew::prelude::*;

//use crate::file::FileDetails;

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    pub selected_file: Option<String>,
}

#[function_component(FileView)]
pub fn file_view(props: &FileViewProps) -> Html {
    let selected_file = props.selected_file.clone();

    if let Some(selected_file) = selected_file {
        html! {
            <p>
                {format!("Selected file: {:#?}", selected_file)}
            </p>
        }
    } else {
        html! {
            <p>
                {"No file selected."}
            </p>
        }
    }
}
