use std::path::Path;

use acridotheres::{archive::extract::extract_all, formats::Format};

pub fn run(file: String, output: Option<String>, format: Option<String>, buffer_size: u64) {
    let output = output.unwrap_or_else(|| file.clone().split('.').next().unwrap().to_string());
    let format = format.unwrap_or_else(|| "auto".to_string());

    extract_all(
        Path::new(&file),
        Path::new(&output),
        Format::Zip,
        buffer_size,
    )
    .unwrap();
}
