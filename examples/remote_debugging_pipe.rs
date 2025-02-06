use std::fs::File;
use std::io::prelude::*;
use std::os::fd::{AsRawFd, RawFd};


#[tokio::main]
async fn main() {
    let mut file = File::open("input.json").unwrap();
    println!("{:?}", file.as_raw_fd());
    let mut file = File::open("input.json").unwrap();
    println!("{:?}", file.as_raw_fd());
}
