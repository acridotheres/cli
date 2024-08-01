pub fn version() {
    println!("Acridotheres\n");
    println!("cli v{}", env!("CARGO_PKG_VERSION"));
    println!("core v{}", corelib::get_version());
}