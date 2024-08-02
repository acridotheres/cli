use corelib::{
    archive::{self, OriginalArchiveMetadata},
    formats::{self, Formats},
};

pub fn metadata(format_string: String, input: String, check_integrity: bool, buffer_size: u64) {
    let format = formats::from_string(&format_string);
    let metadata = archive::metadata(format, input, check_integrity, buffer_size).unwrap();

    match format {
        Formats::Zip => {
            let OriginalArchiveMetadata::Zip(metadata) = *metadata;

            println!("Type: {}", format_string);
            let mut file_count: u32 = 0;
            let mut dir_count: u32 = 0;
            for file in &metadata.files {
                if file.is_directory {
                    dir_count += 1;
                    continue;
                }
                file_count += 1;
            }
            println!("Files: {}", file_count);
            println!("Directories: {}", dir_count);
            let mut total_size = 0;
            for file in &metadata.files {
                total_size += file.uncompressed_size;
            }
            let total_size = byte_unit::Byte::from_u64(total_size.into())
                .get_appropriate_unit(byte_unit::UnitType::Decimal);
            println!("Total size (uncompressed): {}", total_size);
            let mut total_size = 0;
            for file in &metadata.files {
                total_size += file.size;
            }
            let total_size = byte_unit::Byte::from_u64(total_size)
                .get_appropriate_unit(byte_unit::UnitType::Decimal);
            println!("Total size (compressed): {}", total_size);
        }
    }
}
