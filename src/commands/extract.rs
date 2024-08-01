use corelib::{archive, formats};

pub fn extract(
    format: String,
    input: String,
    output: String,
    index: Option<u32>,
    path: Option<String>,
    all: bool,
    check_integrity: bool,
    buffer_size: u64,
) {
    archive::extract(
        formats::from_string(&format),
        input,
        output,
        index,
        path,
        all,
        check_integrity,
        buffer_size,
    )
    .unwrap();
}
