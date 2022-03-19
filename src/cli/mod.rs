use std::io::{stdin, stdout, Stdin, Stdout, Write};

pub struct CLI {
    pub stdin: Stdin,
    pub stdout: Stdout,
}

impl CLI {
    pub fn new() -> CLI {
        Self {
            stdin: stdin(),
            stdout: stdout(),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> () {
        self.stdout.write(buf).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn read(&mut self, buf: &mut String) {
        self.stdin.read_line(buf).unwrap();
    }
}
