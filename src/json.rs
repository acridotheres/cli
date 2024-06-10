use corelib::get_version;

pub fn get_ver() {
  print!("{{\n\t");
  print!("\"cli\": \"v{}\",\n\t", env!("CARGO_PKG_VERSION"));
  print!("\"core\": \"v{}\"\n", get_version());
  print!("}}\n");
}