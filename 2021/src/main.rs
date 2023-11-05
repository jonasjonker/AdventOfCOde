use std::error;
use std::path::Path;

mod utils;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = Path::new("data/2021/1/input.txt");

    utils::read_all(path)?;
    Ok(())
}