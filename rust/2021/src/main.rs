use std::error;
use std::path::Path;

mod utils;
mod crate::year2021::day01;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = Path::new("data/day01.input");

    day01::part_1();
    utils::read(path)?;
    Ok(())
}