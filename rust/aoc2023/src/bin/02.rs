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
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}

#[derive(Debug, Default)]
struct Game {
    id: usize,
    rounds: Vec<Round>
}

fn parse_games(file_content: &String) -> Result<Vec<Game>, Box<dyn Error>> { 
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
    let games = file_content
        .lines()
        .map(|line| {
            let mut game = Game::default();
            let mut rounds = vec![];

            for (i, token) in line.split(&[';', ':']).enumerate() {
                if i == 0 {
                    game.id = token.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
                } else {
                    let round = re
                        .captures_iter(token)
                        .map(|caps| {
                            let (_, [count, color]) = caps.extract();
                            (color, count.parse::<usize>().unwrap_or_default())
                        })
                        .fold(Round::default(), |acc, (color, count)| match color {
                            "red"   => acc + Round { r: count, g: 0, b: 0 },
                            "green" => acc + Round { r: 0, g: count, b: 0 },
                            "blue"  => acc + Round { r: 0, g: 0, b: count },
                            _ => acc,
                        });
                    rounds.push(round);
                }
            }

            let max_attrs_round = rounds
                .iter()
                .fold(Round::default(), |acc, x| x.max_attrs(acc));
            game.rounds.push(max_attrs_round);
            game
        })
        .collect();
    Ok(games)
}

fn part_one(games: &Vec<Game>) -> usize { 
    let max_attrs = Round { r: 12, g: 13, b: 14 };
    games
        .iter()
        .filter(|g| { g.rounds[0].issubset(max_attrs) })
        .fold(0, |acc, g| acc + g.id)
}


fn part_two(games: &Vec<Game>) -> usize { 
    games
        .iter()
        .fold(0, |acc, g| {
            let r = g.rounds[0];
            acc + (r.r * r.g * r.b)
        })
}


fn main() -> Result<(), Box<dyn Error>> {
    let path = "../../data/2023/2/input.txt";
    let file = fs::read_to_string(path).unwrap();

    let games = parse_games(&file)?;
    let answer_part_one = part_one(&games);
    let answer_part_two = part_two(&games);

    println!("{answer_part_one}");
    println!("{answer_part_two}");

    Ok(())
}