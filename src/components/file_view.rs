use web_sys::{File, Url};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    pub selected_file: Option<File>,
}

#[function_component(FileView)]
pub fn file_view(props: &FileViewProps) -> Html {
    let selected_file = props.selected_file.clone();

    if let Some(selected_file) = selected_file {
        //let view_file = selected_file;
        let url = Url::create_object_url_with_blob(&selected_file).unwrap();

        html! {
            <div>
                    <iframe
                        class={classes!("mt-4", "p-8", "w-full", "h-screen", "bg-white")}
                        src={url}
                    />
            </div>
            // needs to be added later to avoid memory leaks
            // { Url::revoke_object_url(url.as_ref().unwrap()).unwrap(); }
        }
    } else {
        html! {
            <p
                class={classes!("mt-4", "mx-4", "w-full")}
            >
                {"No file selected."}
            </p>
        }
    }
}
