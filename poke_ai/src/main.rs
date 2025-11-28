use std::io::{self,Write};

pub fn main() {
    io::stdout().write_all(b"balls").unwrap();
}