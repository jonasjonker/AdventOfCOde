use std::fs;
use std::error::Error;
use regex::Regex;

fn part_one(file: &String) -> Result<(), Box<dyn Error>> { 
    let ans = file.lines()
        .map(|line| line.chars())
        .map(|chars|
            chars
                .map(|char| char.to_digit(10))
                .flatten()
                .collect::<Vec<u32>>()
        )
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

    let regexes = vec![
        (Regex::new(r"zero").unwrap(), "0"),
        (Regex::new(r"one").unwrap(), "1"),
        (Regex::new(r"two").unwrap(), "2"),
        (Regex::new(r"three").unwrap(), "3"),
        (Regex::new(r"four").unwrap(), "4"),
        (Regex::new(r"five").unwrap(), "5"),
        (Regex::new(r"six").unwrap(), "6"),
        (Regex::new(r"seven").unwrap(), "7"),
        (Regex::new(r"eight").unwrap(), "8"),
        (Regex::new(r"nine").unwrap(), "n"),
        (Regex::new(r"ten").unwrap(), "10"),
        (Regex::new(r"eleven").unwrap(), "11"),
        (Regex::new(r"twelve").unwrap(), "12"),
        (Regex::new(r"thirteen").unwrap(), "13"),
        (Regex::new(r"fourteen").unwrap(), "14"),
        (Regex::new(r"fifteen").unwrap(), "15"),
        (Regex::new(r"sixteen").unwrap(), "16"),
        (Regex::new(r"seventeen").unwrap(), "17"),
        (Regex::new(r"eightteen").unwrap(), "18"),
        (Regex::new(r"nineteen").unwrap(), "19"),
        (Regex::new(r"twenty").unwrap(), "2"),
        (Regex::new(r"thirty").unwrap(), "3"),
        (Regex::new(r"forty").unwrap(), "4"),
        (Regex::new(r"fifty").unwrap(), "5"),
        (Regex::new(r"sixty").unwrap(), "6"),
        (Regex::new(r"seventy").unwrap(), "7"),
        (Regex::new(r"eighty").unwrap(), "8"),
        (Regex::new(r"ninety").unwrap(), "9"),
        (Regex::new(r"(?<![0-9])hundred").unwrap(), "1"),
        (Regex::new(r"(?<![0-9])thousand").unwrap(), "1"),
    ];


    for (re, v) in regexes.iter() {
        processed_file = re.replace_all(processed_file, v);
    }
    processed_file
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../../data/2023/1/test_2.txt";
    let file = fs::read_to_string(path).unwrap();

    let _ = part_one(&file);

    let processed_file = part_two(&file);
    let _ = part_one(&processed_file);



    // println!("Finished with: {ans}");
    Ok(())
}