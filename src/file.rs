use std::{io::Read, path::Path};

use mime_guess::from_ext;
use web_sys::{Blob, File};
use zip::read::ZipFile;

pub fn zip2js(mut zip_file: ZipFile) -> File {
    let mut buffer = Vec::new();
    zip_file.read_to_end(&mut buffer).unwrap();
    let array = js_sys::Uint8Array::from(&buffer[..]);
    let blob = Blob::new_with_u8_array_sequence(&array).unwrap();
    let property_bag = web_sys::FilePropertyBag::new();
    property_bag.set_type(&get_mime_type(zip_file.name().to_string()));
    let web_file = File::new_with_blob_sequence_and_options(
        &js_sys::Array::of1(&blob),
        zip_file.name(),
        &property_bag
    ).unwrap();
    web_file
}

fn get_mime_type(file_name: String) -> String {
    let path = Path::new(&file_name);
    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        from_ext(ext).first_or_octet_stream().essence_str().to_string()
    } else {
        "application/octet-stream".to_string()
    }
}