use std::io::Cursor;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{File, FileReader};
use js_sys::Uint8Array;
use yew::prelude::*;
use zip::ZipArchive;

use crate::components::file_list::FileList;
use crate::components::file_view::FileView;

#[derive(Properties, PartialEq)]
pub struct ArchiveViewProps {
    pub archive: Option<File>,
}

#[function_component(ArchiveView)]
pub fn archive_view(props: &ArchiveViewProps) -> Html {
    let file_names = use_state(|| vec![]);

    {
        let file_names = file_names.clone();
        let archive = props.archive.clone();
        
        use_effect_with(archive, move |archive| {
            if let Some(archive) = archive {
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
                reader.read_as_array_buffer(&archive).unwrap();
                onloadend.forget();
            }
        });
    }

    html! {
        <div>
            {
                if !file_names.is_empty() {
                    html! {
                        <div class={classes!("grid", "grid-cols-2")}>
                            <FileList file_names={(*file_names).clone()} />
                            <FileView />
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
