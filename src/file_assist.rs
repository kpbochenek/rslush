use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;

pub fn open_file(path_name: &str) -> String {
    let mut file = File::open(path_name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn save_file(path_name: &str, content: &Vec<String>) {
    println!("Trying to save file {}", path_name);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path_name)
        .unwrap();
    for l in content {
        file.write_all(l.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
    println!("File saved {}", path_name);
}
