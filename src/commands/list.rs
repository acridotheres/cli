use corelib::{
    archive::{self, OriginalArchiveMetadata},
    formats::{self, Formats},
};

pub fn list(format_string: String, input: String, check_integrity: bool, buffer_size: u64) {
    let format = formats::from_string(&format_string);
    let metadata = archive::metadata(format, input, check_integrity, buffer_size).unwrap();

    match format {
        Formats::Zip => {
            let OriginalArchiveMetadata::Zip(metadata) = *metadata;

            println!();
            for (i, file) in (0_u32..).zip(metadata.files.iter()) {
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
                        byte_unit::Byte::from_u64(file.size)
                            .get_appropriate_unit(byte_unit::UnitType::Decimal)
                    );
                    println!(
                        "Last modified: {}",
                        file.modified.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("CRC-32 checksum: 0x{:x}", file.checksum);
                    println!("Compression method: {}\n", file.compression);
                };
            }
        }
    }
}
