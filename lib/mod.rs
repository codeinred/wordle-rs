use std::vec::Vec;
use std::fs::File;
use std::io::{BufReader, Result, BufRead};
use std::path::Path;

mod types;

pub use types::*;

pub fn read_shit<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

pub fn hello_world() {
    println!("Hello, world!")
}
