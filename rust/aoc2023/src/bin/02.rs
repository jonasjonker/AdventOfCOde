use std::fs;
use std::error::Error;
use std::ops::Add;
use std::cmp::max;
use regex::Regex;

#[derive(Debug, Default, Copy, Clone)]
struct Round {
    r: usize,
    g: usize,
    b: usize,
}

impl Add for Round {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Round {
    /// Returns `Round` with the maximum value for each attribute between `self` and `other`
    fn max_attrs(self, other: Self) -> Self {
        Self {
            r: max(self.r, other.r),
            g: max(self.g, other.g),
            b: max(self.b, other.b),
        }
    }

    /// Checks if all attribues of `self` are lower or equal to the attributes of `other`
    fn issubset(self, other: Self) -> bool {
        if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    rounds: Vec<Round>
}

fn parse_games(file: &String) -> Result<Vec<Game>, Box<dyn Error>> { 
    Ok(file.lines()
        // split Game and Rounds
        .map(|s| s.split(&[';', ':']).collect::<Vec<_>>())
        // Parse Game & its Rounds
        .map(|v| Game {
            id: v[0].split(' ').collect::<Vec<_>>()[1].parse().unwrap(),
            rounds: 
                v[1..].iter()
                // use regex to capture color counts
                .map(|h| {
                    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
                    re.captures_iter(h)
                        .map(|caps| {
                            let (_, [count, color]) = caps.extract();
                            (color, count.parse::<usize>().unwrap_or_default())
                        })
                        .collect::<Vec<(&str,usize)>>()
                })
                // map color counts to a Round struct
                .map(|v| 
                    v.iter().fold(Round::default(), |acc, (color, count)| match (*color, *count) {
                        ("red",   n) => acc + Round { r:n, g:0, b:0 },
                        ("green", n) => acc + Round { r:0, g:n, b:0 },
                        ("blue",  n) => acc + Round { r:0, g:0, b:n },
                        _ => acc
                    })
                )
                .collect()
            }
        )
        // find max values for r, g, b values
        .map(|g| Game {
            id: g.id,
            rounds: vec![
                g.rounds.iter().fold(Round::default(), |acc, x| x.max_attrs(acc))
            ]
        }).collect::<Vec<_>>()
    )
}

fn part_one(games: &Vec<Game>) -> Result<(), Box<dyn Error>> { 
        // Filter records and sum id's
    let ans = games.iter()
        .filter(|g| {
            let check = Round { r: 12, g: 13, b: 14 };
            g.rounds[0].issubset(check)
        })
        .fold(0, |acc, g| acc + g.id);
    println!("{ans}");
    Ok(())
}


fn part_two(games: &Vec<Game>) -> Result<(), Box<dyn Error>> { 
        // Filter records and sum id's
    let ans = games.iter()
        .fold(0, |acc, g| {
            let r = g.rounds[0];
            acc + (r.r * r.g * r.b)
        });
    
    println!("{ans}");
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let path = "../../data/2023/2/input.txt";
    let file = fs::read_to_string(path).unwrap();

    let games = parse_games(&file)?;
    let _ = part_one(&games);
    let _ = part_two(&games);

    Ok(())
}