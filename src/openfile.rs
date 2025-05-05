use web_sys::{wasm_bindgen::{JsCast, JsValue}, HtmlInputElement};
use leptos::{prelude::*, ev::Event};

#[component]
pub fn OpenFile() -> impl IntoView {
    let change_file_upload = move |e: Event| {
        let files = e.target().unwrap().unchecked_ref::<web_sys::HtmlInputElement>().files().unwrap();
        web_sys::console::log_1(&files);
        // web_sys::console::log_1(&JsValue::from_str("Check!"));
        // let input :HtmlInputElement = e.unchecked_into();
        // let files = input.files();
        // if let Some(ref files) = files {
        //     web_sys::console::log_1(&JsValue::from_str("Check 2!"));
        //     let file = files.item(0).unwrap();
        //     let name = file.name();
        //     web_sys::console::log_2(
        //         &JsValue::from_str("Found file"),
        //         &JsValue::from_str(&name)
        //     )
        // }
        // if files == None {
        //     web_sys::console::log_1(&JsValue::from_str("files == None"));
        // }
    };

    view! {
        <p class="w-fit mx-auto mt-5 p-1 border-2 rounded-md border-dotted border-stonde-50 bg-linear-to-b from-stone-50 to-stone-300 text-center">
            <label for="file-upload">
                //<div 
                    //class="mx-12 mt-4 p-6 flex border-2 border-dotted rounded border-stone-50 bg-indigo-600 justify-center items-center">
                    //ondrop={on_drop}
                    //ondragover={Callback::from(|event: DragEvent| {
                    //    event.prevent_default();
                    //})}
                    //ondragenter={Callback::from(|event: DragEvent| {
                    //    event.prevent_default();
                    //})}
                //>
                    "Datei hier hineinziehen oder klicken, um auszuwählen."
                //</div>
            </label>
            <input
                id="file-upload"
                class="hidden"
                type="file"
                accept="application/zip"
                on:change=change_file_upload
            />
        </p>
    }
}

// use web_sys::{DragEvent, File, HtmlInputElement};

// #[derive(Properties, PartialEq)]
// pub struct OpenArchiveProps {
//     pub archive: UseStateHandle<Option<File>>,
// }

// #[function_component(OpenArchive)]
// pub fn open_archive(props: &OpenArchiveProps) -> Html {
//     let on_drop = {
//         let archive = props.archive.clone();
//         Callback::from(move |e: DragEvent| {
//             e.prevent_default();
//             let files = e.data_transfer().unwrap().files();
//             if let Some(file_list) = files {
//                 archive.set(file_list.get(0));           
//             }           
//         })
//     };
//     let on_file_change = {
//         let archive = props.archive.clone();
//         Callback::from(move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let files = input.files();
//             if let Some(file_list) = files {
//                 archive.set(file_list.get(0));           
//             }
//         })
//     };