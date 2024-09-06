use web_sys::{DragEvent, File, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OpenArchiveProps {
    pub archive: UseStateHandle<Option<File>>,
}

#[function_component(OpenArchive)]
pub fn open_archive(props: &OpenArchiveProps) -> Html {
    let on_drop = {
        let archive = props.archive.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            let files = e.data_transfer().unwrap().files();
            if let Some(file_list) = files {
                archive.set(file_list.get(0));           
            }           
        })
    };
    let on_file_change = {
        let archive = props.archive.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = input.files();
            if let Some(file_list) = files {
                archive.set(file_list.get(0));           
            }
        })
    };

    html! {
        <>
            <label for="file-upload">
                <div 
                    class={classes!("mx-12", "mt-4", "p-6", "flex", "border-2", "border-dotted", "rounded", "border-stone-50", "bg-indigo-600", "justify-center", "items-center")}
                    ondrop={on_drop}
                    ondragover={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}
                    ondragenter={Callback::from(|event: DragEvent| {
                        event.prevent_default();
                    })}
                >
                <p class={classes!("text-center")}>{"Datei hier hineinziehen oder klicken, um auszuwählen."}</p>
                </div>
            </label>
            <input
                id="file-upload"
                class={classes!("hidden")}
                type="file"
                accept="application/zip"
                onchange={on_file_change}
            />
        </>
    }
}