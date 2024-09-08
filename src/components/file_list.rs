use web_sys::{File, HtmlUListElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub files: Vec<File>,
    pub selected_file: UseStateHandle<Option<File>>,
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
        <div
            class={classes!("mt-4", "p-8")}
        >
            <ul>
                {
                    for files.iter().enumerate().map(|(i,file)| html! {
                        <li
                            id={format!("{}", i)}
                            onclick={on_click.clone()}
                            key={i}  
                        >
                            { file.name() }
                        </li>
                    })
                }
            </ul>
        </div>
    }
}