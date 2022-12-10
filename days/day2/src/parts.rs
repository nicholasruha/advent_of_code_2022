use std::fs;
use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}
pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl Hand {
    pub fn from(item: &str) -> Self {
        match item {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid"),
        }
    }

    fn from_into_iter<'a>(items: impl IntoIterator<Item = &'a str>) -> (Self, Self) {
        let a: Vec<Hand> = items.into_iter().map(|item| Hand::from(item)).collect();
        (a[0], a[1])
    }
    fn into_score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

fn round_decider(other_hand: &Hand, round_outcome: &HandResult) -> Hand {
    match round_outcome {
        HandResult::Win => other_hand.beats().beats(),
        HandResult::Loss => other_hand.beats(),
        HandResult::Draw => other_hand.clone(),
    }
}

enum HandResult {
    Win,
    Loss,
    Draw,
}

impl HandResult {
    fn from(result: &str) -> Self {
        match result {
            "X" => HandResult::Loss,
            "Y" => HandResult::Draw,
            "Z" => HandResult::Win,
            _ => panic!("result out of bounds"),
        }
    }
}

pub fn part1() -> usize {
    // Correct = 8392
    
    let mut score = 0;
    if let Ok(lines) = read_lines("./days/day2/day2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (other_hand, own_hand) = Hand::from_into_iter(ip.split(" "));
                let (other_beats, own_beats) = (other_hand.beats(), own_hand.beats());
                match (other_beats, own_beats) {
                    _ if own_beats == other_hand => {
                        // Win
                        score += 6 + own_hand.into_score();
                    }
                    _ if other_beats == own_hand => {
                        // Lose
                        score += 0 + own_hand.into_score();
                    }
                    _ => {
                        score += 3 + own_hand.into_score();
                    }
                }
            }
        }
        println!("{}", score);
    }
    score
}

pub fn part2() -> usize {
    let mut score = 0;
    if let Ok(lines) = read_lines("./days/day2/day2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let p: Vec<&str> = ip.split(" ").into_iter().collect();
                let (other_hand, result) = (Hand::from(p[0]), HandResult::from(p[1]));
                let own_hand = round_decider(&other_hand, &result);
                let (other_beats, own_beats) = (other_hand.beats(), own_hand.beats());
                match (other_beats, own_beats) {
                    _ if own_beats == other_hand => {
                        // Win
                        println!("WON");
                        score += 6 + own_hand.into_score();
                    }
                    _ if other_beats == own_hand => {
                        // Lose
                        println!("LOSS");
                        score += 0 + own_hand.into_score();
                    }
                    _ => {
                        println!("DRAW");
                        score += 3 + own_hand.into_score();
                    }
                }
            }
        }
    }
    score
}
