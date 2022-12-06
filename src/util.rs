use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;

// Helper from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_string<P>(filename: P) -> Result<String, Box<dyn std::error::Error>>
where P: AsRef<Path>, {
    let data = fs::read_to_string(filename)?;
    Ok(data)
}