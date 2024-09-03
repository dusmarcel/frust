use std::io::Cursor;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{File, FileReader};
use js_sys::Uint8Array;
use yew::prelude::*;
use zip::ZipArchive;

#[derive(Properties, PartialEq)]
pub struct FileListProps {
    pub file: Option<File>,
}

#[function_component(FileView)]
pub fn file_view(props: &FileListProps) -> Html {
    let file_names = use_state(|| vec![]);

    {
        let file_names = file_names.clone();
        let file = props.file.clone();
        
        use_effect_with(file, move |file| {
            if let Some(file) = file {
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();

                let onloadend = Closure::wrap(Box::new(move || {
                    let array_buffer = reader_clone.result().unwrap();
                    let array = Uint8Array::new(&array_buffer);
                    let cursor = Cursor::new(array.to_vec());

                    if let Ok(mut archive) = ZipArchive::new(cursor) {
                        let mut names = vec![];
                        for i in 0..archive.len() {
                            if let Ok(file) = archive.by_index(i) {
                                names.push(file.name().to_string());
                            }
                        }
                        file_names.set(names);
                    } else {
                        web_sys::console::error_1(&"Failed to open ZIP archive".into());
                    }
                }) as Box<dyn FnMut()>);

                reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onloadend.forget();
            }
        });
    }

    let on_click = {
        Callback::from(move |_e: MouseEvent| {
            web_sys::console::log_1(&"Click!".into());
        })
    };

    html! {
        <div>
            {
                if !file_names.is_empty() {
                    html! {
                        <div class={classes!("grid", "grid-cols-2")}>
                            <ul>
                                { for file_names.iter().map(|name| html! {
                                    <li onclick={on_click.clone()}>
                                        { name }
                                    </li> })
                                }
                            </ul>
                            <p>
                                {"Hallo, Grid!"}
                            </p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
