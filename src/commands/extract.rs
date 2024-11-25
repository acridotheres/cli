use acridotheres::{
    archive::extract::extract_all,
    formats::{
        auto::{from_str, DetectionAccuracy},
        Format,
    },
};
use dh::recommended::*;
use std::path::Path;

pub fn run(
    file: String,
    output: Option<String>,
    format: Option<String>,
    buffer_size: u64,
    autodetection_method: String,
    password: Option<String>,
) {
    let output = output.unwrap_or_else(|| file.clone().split('.').next().unwrap().to_string());
    let format_arg = format.clone();
    let format = from_str(&format.unwrap_or_else(|| "auto".to_string()));
    let format = format.unwrap_or_else(|| {
        if format_arg.is_some() && format_arg.as_ref().unwrap() != "auto" {
            eprintln!(
                "Format {} is unknown, falling back to auto detection",
                format_arg.unwrap_or_else(|| "auto".to_string())
            );
        }

        let method = match autodetection_method.as_str() {
            "extension" => DetectionAccuracy::Extension,
            "magic" => DetectionAccuracy::Magic,
            "parse" => DetectionAccuracy::Parse,
            _ => {
                eprintln!(
                    "Autodetection method {} is unknown, falling back to extension",
                    autodetection_method
                );
                DetectionAccuracy::Extension
            }
        };

        let mut reader = dh::file::open_r(&file).unwrap();
        Format::auto(Path::new(&file), &mut reader, &method).unwrap()
    });

    extract_all(
        Path::new(&file),
        Path::new(&output),
        format,
        buffer_size,
        password.as_deref(),
    )
    .unwrap();
}
