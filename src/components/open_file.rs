use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OpenFileProps {
    pub file_name: UseStateHandle<Option<String>>,
}

#[function_component(OpenFile)]
pub fn open_file(props: &OpenFileProps) -> Html {
    let on_file_change = {
        let file_name = props.file_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = input.files();
            if let Some(file_list) = files {
                if let Some(file) = file_list.get(0) {
                    file_name.set(Some(file.name()))
                }               
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