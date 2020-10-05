use std::fs::File;
use std::io::Read;

pub fn open_file(path_name: &str) -> String {
    let mut file = File::open(path_name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
