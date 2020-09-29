use std::fs::File;
use std::io::prelude::*;

pub fn open_test() -> String {
    let mut file = File::open("src/example.kis").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
