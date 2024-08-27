use web_sys::{HtmlInputElement, File};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OpenFileProps {
    pub file: UseStateHandle<Option<File>>,
}

#[function_component(OpenFile)]
pub fn open_file(props: &OpenFileProps) -> Html {
    let on_file_change = {
        let file = props.file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = input.files();
            if let Some(file_list) = files {
                file.set(file_list.get(0));           
            }
        })
    };

    html! {
        <input
            id="file-upload"
            type="file"
            accept="application/zip"
            onchange={on_file_change}
        />
    }
}