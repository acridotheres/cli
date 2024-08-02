use corelib::{
    archive::{self, EntrySource},
    file::FsFile,
    formats,
};

pub fn create(format: String, input: String, output: String, buffer_size: u64) {
    let files: Vec<EntrySource> = input
        .split(';')
        .map(|file| {
            let file = file.split(':').collect::<Vec<&str>>();
            let source_path = file.get(0).unwrap();
            let mut target_path = source_path;
            match file.get(1) {
                Some(path) => {
                    target_path = path;
                }
                None => {}
            }
            EntrySource {
                path: target_path.to_string(),
                source: FsFile::new(&source_path.to_string()),
            }
        })
        .collect();
    archive::create(formats::from_string(&format), output, files, buffer_size).unwrap();
}
