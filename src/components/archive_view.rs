use std::io::Cursor;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{File, FileReader};
use js_sys::Uint8Array;
use yew::prelude::*;
use zip::ZipArchive;

use crate::file::FileDetails;
use crate::components::file_list::FileList;
use crate::components::file_view::FileView;

#[derive(Properties, PartialEq)]
pub struct ArchiveViewProps {
    pub archive: Option<File>,
}

#[function_component(ArchiveView)]
pub fn archive_view(props: &ArchiveViewProps) -> Html {
    let files = use_state(|| vec![]);
    let selected_file = use_state(|| None);

    {
        let files = files.clone();
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
                        let mut files_vec = vec![];
                        for i in 0..archive.len() {
                            if let Ok(file) = archive.by_index(i) {
                                files_vec.push(FileDetails {
                                    id: i,
                                    name: file.name().to_string(),
                                    file_type: mime_guess::from_ext(file.name()).first_or_octet_stream().essence_str().to_string(),
                                    //data: array.to_vec(),
                                });
                            }
                        }
                        files.set(files_vec);
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
                if !files.is_empty() {
                    html! {
                        <div class={classes!("grid", "grid-cols-2")}>
                            <FileList
                                files={(*files).clone()}
                                selected_file={(selected_file).clone()}
                            />
                            <FileView
                                selected_file={(*selected_file).clone()}
                            />
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
