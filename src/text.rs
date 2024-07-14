use byte_unit;
use corelib;

pub fn get_version() {
    println!("Acridotheres\n");
    println!("cli v{}", env!("CARGO_PKG_VERSION"));
    println!("core v{}", corelib::get_version());
}

pub fn zip_metadata(path: &str) {
    let mut file = corelib::FileReader::new(path);

    let metadata = corelib::formats::zip::parser::metadata(&mut file);

    println!("Type: {}", metadata.archive.format);
    let mut file_count: u32 = 0;
    let mut dir_count: u32 = 0;
    for file in &metadata.files {
        if file.file.is_directory {
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
        total_size += file.file.size;
    }
    let total_size = byte_unit::Byte::from_u64(total_size.into())
        .get_appropriate_unit(byte_unit::UnitType::Decimal);
    println!("Total size (compressed): {}", total_size);
}

pub fn zip_list(path: &str) {
    let mut file = corelib::FileReader::new(path);

    let metadata = corelib::formats::zip::parser::metadata(&mut file);

    let mut i: u32 = 0;
    println!("");
    for file in &metadata.files {
        println!("{}", file.file.path);
        println!("{}", "=".repeat(file.file.path.len()));
        println!("Index: {}", i);
        if file.file.is_directory {
            println!("Directory\n");
        } else {
            println!(
                "Size: {} ({} compressed)",
                byte_unit::Byte::from_u64(file.uncompressed_size.into())
                    .get_appropriate_unit(byte_unit::UnitType::Decimal),
                byte_unit::Byte::from_u64(file.file.size.into())
                    .get_appropriate_unit(byte_unit::UnitType::Decimal)
            );
            println!("Last modified: {}", file.file.modified.format("%Y-%m-%d %H:%M:%S"));
            println!("CRC-32 checksum: 0x{:x}", file.checksum);
            println!("Compression method: {}\n", file.compression);
        };
        i += 1;
    }
}

pub fn zip_extract(input: &str, output: &str, index: Option<u32>, path: Option<String>, all: bool) {
    let mut file = corelib::FileReader::new(input);
    std::fs::create_dir_all(output).unwrap();

    let metadata = corelib::formats::zip::parser::metadata(&mut file);

    if all {
        corelib::formats::zip::parser::extract(&mut file, &metadata.files, 1024, &|path| {
            format!("{}/{}", output, path)
        });
    } else {
        if index != None {
            let index = index.unwrap();
            if index >= metadata.files.len() as u32 {
                println!("Index out of range");
                return;
            }
            let files = &vec![corelib::ZipFileEntry {
                file: corelib::FileEntry {
                    path: metadata.files[index as usize].file.path.clone(),
                    size: metadata.files[index as usize].file.size,
                    offset: metadata.files[index as usize].file.offset,
                    modified: metadata.files[index as usize].file.modified,
                    is_directory: metadata.files[index as usize].file.is_directory,
                },
                uncompressed_size: metadata.files[index as usize].uncompressed_size,
                checksum: metadata.files[index as usize].checksum,
                extra_field: metadata.files[index as usize].extra_field.clone(),
                version: metadata.files[index as usize].version,
                bit_flag: metadata.files[index as usize].bit_flag,
                compression: metadata.files[index as usize].compression,
            }];
            corelib::formats::zip::parser::extract(&mut file, files, 1024, &|path| {
                format!("{}/{}", output, path)
            });
        } else {
            let path = path.unwrap();
            let files: Vec<corelib::ZipFileEntry> = metadata
                .files
                .iter()
                .filter_map(|file| if file.file.path.starts_with(&path) { Some(corelib::ZipFileEntry {
                    file: corelib::FileEntry {
                        path: file.file.path.clone(),
                        size: file.file.size,
                        offset: file.file.offset,
                        modified: file.file.modified,
                        is_directory: file.file.is_directory,
                    },
                    uncompressed_size: file.uncompressed_size,
                    checksum: file.checksum,
                    extra_field: file.extra_field.clone(),
                    version: file.version,
                    bit_flag: file.bit_flag,
                    compression: file.compression,
                }) } else { None })
                .collect();
            corelib::formats::zip::parser::extract(&mut file, &files, 1024, &|path| {
                format!("{}/{}", output, path)
            });
        }
    };
}
