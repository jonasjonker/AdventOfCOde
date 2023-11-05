use std::{fs,error};
use std::str::Lines;
use std::path::Path;

pub fn read_all(path: &Path) -> Result<Lines<'_>, Box<dyn error::Error>> {
    Ok(
        fs::read_to_string(path)
        .expect("File not found!")
        .lines()
    )
}