use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn use_metals() {
    println!("Spawning metals");
    let mut child = Command::new("metals-emacs")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start metals");
    let stdout = child.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);
    reader.lines();
    println!("Done metals: {} ", out);
}
