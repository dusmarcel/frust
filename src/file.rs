use std::io::Read;

use web_sys::{Blob, File};
use zip::read::ZipFile;

pub fn zip2js(mut zip_file: ZipFile) -> File {
    let mut buffer = Vec::new();
    zip_file.read_to_end(&mut buffer).unwrap();
    let array = js_sys::Uint8Array::new_with_length(buffer.len() as u32);
    array.copy_from(&buffer[..]);
    let blob = Blob::new_with_u8_array_sequence(&array).unwrap();
    let web_file = File::new_with_blob_sequence_and_options(
        &js_sys::Array::of1(&blob),
        zip_file.name(),
        &web_sys::FilePropertyBag::new()
    ).unwrap();
    web_file
}