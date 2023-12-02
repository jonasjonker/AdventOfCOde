use std::fs;
use std::error::Error;

fn part_one(file_content: &String) -> Result<u32, Box<dyn Error>> { 
    let sum = file_content
        .lines()
        // drop chars in string that don't convert to digits.
        .map(|line| 
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        )
        // concat first and last digit and interpret as number
        .filter_map(|digits| {
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                format!("{first}{last}").parse::<u32>().ok()
            } else {
                None
            }
        })
        .sum::<u32>();
    Ok(sum)
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

    for (key, val) in &trans {
        processed_file = processed_file.replace(key, val);
    }
    processed_file
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../../data/2023/1/input.txt";
    let file_content = fs::read_to_string(path)?;

    let answer_part_one = part_one(&file_content)?;
    let processed_file_content = part_two(&file_content);
    let answer_part_two = part_one(&processed_file_content)?;

    println!("{answer_part_one}");
    println!("{answer_part_two}");

    Ok(())
}