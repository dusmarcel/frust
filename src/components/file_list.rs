use web_sys::HtmlUListElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub file_names: Vec<String>,
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let file_names = props.file_names.clone();

    let on_click = {
        Callback::from(move |e: MouseEvent| {
            let list_element: HtmlUListElement = e.target_unchecked_into();
            web_sys::console::log_1(&format!("Clicked: {}", list_element.inner_text()).into())
        })
    };

    html! {
        <div>
            <ul>
                { for file_names.iter().map(|name| html! {
                    <li onclick={on_click.clone()}>
                        { name }
                    </li> })
                }
            </ul>
        </div>
    }
}