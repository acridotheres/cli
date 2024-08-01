use corelib::{
    archive::{self, OriginalArchiveMetadata},
    formats::{self, Formats},
};

pub fn list(format_string: String, input: String, check_integrity: bool, buffer_size: u64) {
    let format = formats::from_string(&format_string);
    let metadata = archive::metadata(format, input, check_integrity, buffer_size).unwrap();

    match format {
        Formats::Zip => {
            let metadata = match *metadata {
                OriginalArchiveMetadata::Zip(metadata) => metadata,
            };

            let mut i: u32 = 0;
            println!("");
            for file in &metadata.files {
                println!("{}", file.path);
                println!("{}", "=".repeat(file.path.len()));
                println!("Index: {}", i);
                if file.is_directory {
                    println!("Directory\n");
                } else {
                    println!(
                        "Size: {} ({} compressed)",
                        byte_unit::Byte::from_u64(file.uncompressed_size.into())
                            .get_appropriate_unit(byte_unit::UnitType::Decimal),
                        byte_unit::Byte::from_u64(file.size.into())
                            .get_appropriate_unit(byte_unit::UnitType::Decimal)
                    );
                    println!(
                        "Last modified: {}",
                        file.modified.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("CRC-32 checksum: 0x{:x}", file.checksum);
                    println!("Compression method: {}\n", file.compression);
                };
                i += 1;
            }
        }
    }
}
