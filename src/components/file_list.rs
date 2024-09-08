use web_sys::HtmlUListElement;
use yew::prelude::*;

use crate::file::FileDetails;

#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub files: Vec<FileDetails>,
    pub selected_file: UseStateHandle<Option<FileDetails>>,
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let cb_files = props.files.clone();
    let files = props.files.clone();
    let selected_file = props.selected_file.clone();

    let on_click = {
        Callback::from(move |e: MouseEvent| {
            let list_element: HtmlUListElement = e.target_unchecked_into();
            selected_file.set(cb_files.get(list_element.get_attribute("id").unwrap().parse::<usize>().unwrap()).cloned());
        })
    };

    html! {
        <div>
            <ul>
                {
                    for files.iter().map(|file| html! {
                        <li
                            id={format!("{}", file.id)}
                            onclick={on_click.clone()}
                            key={file.id}  
                        >
                            { file.name.clone() }
                        </li>
                    })
                }
            </ul>
        </div>
    }
}