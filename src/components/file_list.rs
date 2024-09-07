use web_sys::HtmlUListElement;
use yew::prelude::*;

use crate::file::FileDetails;

#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub files: Vec<FileDetails>,
    pub selected_file: UseStateHandle<Option<String>>,
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let files = props.files.clone();
    let selected_file = props.selected_file.clone();

    let on_click = {
        Callback::from(move |e: MouseEvent| {
            let list_element: HtmlUListElement = e.target_unchecked_into();
            selected_file.set(Some(list_element.inner_text()));
        })
    };

    html! {
        <div>
            <ul>
                {
                    for files.iter().map(|file| html! {
                        <li onclick={on_click.clone()}>
                            { file.name.clone() }
                        </li>
                    })
                }
            </ul>
        </div>
    }
}