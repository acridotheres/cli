use corelib;

pub fn get_version() {
    println!("Acridotheres\n");
    println!("cli v{}", env!("CARGO_PKG_VERSION"));
    println!("core v{}", corelib::get_version());
}

pub fn zip_metadata(path: &str) {
    let mut file = corelib::FileReader::new(path);

    println!(
        "metadata {:?}",
        corelib::formats::zip::parser::metadata(&mut file)
    );
}
