use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(File::open(path)?).lines())
}
