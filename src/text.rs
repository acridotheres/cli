use corelib::get_version;

pub fn get_ver() {
  println!("Acridotheres\n");
  println!("cli v{}", env!("CARGO_PKG_VERSION"));
  println!("core v{}", get_version());
}