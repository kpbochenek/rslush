use std::io::Read;
use std::io::Write;
use std::process::Child;
use std::process::{Command, Stdio};

pub struct MetalsProcess {
    process: Child,
}

impl MetalsProcess {
    pub fn new() -> MetalsProcess {
        println!("Spawning metals");
        let mut child: Child = Command::new("metals-emacs")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start metals");
        println!("Metals started");

        MetalsProcess { process: child }
    }

    pub fn write(&mut self, data: &str) {
        let w = self.process.stdin.as_mut().expect("input pipe");
        let len_data = data.len();
        println!("Writing [{}] {}", len_data, data);
        w.write(format!("Content-Length: {}\r\n\r\n", len_data).as_bytes())
            .unwrap();
        w.write_all(data.as_bytes()).unwrap();
        println!("Done! {}", data);
    }

    pub fn read(&mut self) -> Option<String> {
        let mut r = self.process.stdout.as_mut().expect("output pipe");
        let mut buf = [0; 10];
        println!("Reading?");
        let count = r.read(&mut buf).unwrap();
        println!("Received {}", std::str::from_utf8(&buf[..count]).unwrap());
        None
    }
}
