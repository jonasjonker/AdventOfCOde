use std::fs;
use std::error::Error;

fn part_one(file: &String) -> Result<(), Box<dyn Error>> { 
    let ans = file.lines()
        // drop chars in string that don't convert to digits.
        .map(|line| line.chars())
        .map(|chars|
            chars
                .map(|char| char.to_digit(10))
                .flatten() // drops None's and unpacks Some's
                .collect::<Vec<u32>>()
        )
        // concat first and last digit and interpret as number
        .map(|v| match (v.first(), v.last()) {
            (Some(a), Some(b)) => match format!("{a}{b}").parse::<u32>() {
                Ok(n) => Some(n),
                _ => None
            },
            _ => None
        })
        .flatten()
        .sum::<u32>();
    println!("{ans}");
    Ok(())
}

fn part_two(file: &String) -> String {
    let mut processed_file: String = file.clone();

    // prevent breaking overlapping numbers (like oneight or twone) by keeping first and last letter
    let trans = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];


    for (key, val) in trans.iter() {
        processed_file = processed_file.replace(key, val);
    }
    processed_file
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../../data/2023/1/input.txt";
    let file = fs::read_to_string(path)?;

    let _ = part_one(&file);

    let processed_file = part_two(&file);
    let _ = part_one(&processed_file);

    Ok(())
}