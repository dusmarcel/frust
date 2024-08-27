use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    //#[prop_or(AttrValue::from("No file selected"))]
    //pub msg: AttrValue,
    pub file_name: Option<String>,
}

#[function_component(FileView)]
pub fn file_view(props: &FileViewProps) -> Html {
    //let msg = props.msg.clone();

    html! {
        <div>
            {
                if let Some(ref name) = props.file_name {
                    html! { <p>{ format!("Selected file: {}", name) }</p> }
                } else {
                    html! { <p>{ "No file selected!" }</p> }
                }
            }
        </div>
    }
}