use corelib;

pub fn get_version() {
    print!("{{\n\t");
    print!("\"cli\": \"v{}\",\n\t", env!("CARGO_PKG_VERSION"));
    print!("\"core\": \"v{}\"\n", corelib::get_version());
    print!("}}\n");
}
