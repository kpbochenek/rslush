use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;

pub fn open_file(path_name: &str) -> String {
    println!("Opening file: {}", path_name);
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

pub fn list_color_schemes() -> Vec<String> {
    list_files("./colors/", |e| e.ends_with(".json"))
}

fn list_files<F: Fn(&str) -> bool>(directory: &str, filter: F) -> Vec<String> {
    match std::fs::read_dir(directory) {
        Ok(dir_entry) => {
            let c: Vec<std::fs::DirEntry> = dir_entry.filter_map(|e| e.ok()).collect();
            c.iter()
                .map(|x| x.path().to_string_lossy().to_string())
                .filter(|e| filter(e))
                .map(|e| String::from(e.trim_start_matches("./colors/")))
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
