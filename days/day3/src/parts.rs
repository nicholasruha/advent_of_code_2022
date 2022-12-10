use std::collections::{HashMap, HashSet};
use std::fs;

// Soltuon to find numbers by guy in Chris Biscardi day3 2022 video
fn priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
        c @ 'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Unacceptable character {c} !"),
    }
}

// Chris's
pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

pub fn part1() -> u32 {
    // Another way to get letter scores
    let letter_scores = ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let file = fs::read_to_string("./days/day3/day3.txt").unwrap();
    let some_result = file
        .lines()
        .map(|rucksack| {
            let length_of_contents = rucksack.len() / 2;
            let both_halves = rucksack.split_at(length_of_contents);

            let half_set: HashSet<char> =
                HashSet::from_iter(both_halves.0.chars().collect::<Vec<char>>());
            let y = both_halves
                .1
                .chars()
                .find(|character| half_set.contains(character))
                .unwrap();
            if y.is_ascii_lowercase() {
                y as u32 - 96
            } else {
                y as u32 - 38
            }
        })
        .sum::<u32>();
    some_result
}
pub fn part2() -> u32{
    let file = fs::read_to_string("./days/day3/day3.txt").unwrap();
    let mut sum = 0;
    for [a, b, c] in file.lines().array_chunks() {
        let set1: HashSet<char> = HashSet::from_iter(a.chars().collect::<Vec<char>>());
        let set2: HashSet<char> = HashSet::from_iter(b.chars().collect::<Vec<char>>());
        let common_char = c
            .chars()
            .find(|character| set1.contains(character) && set2.contains(character))
            .unwrap();
        if common_char.is_ascii_lowercase() {
            sum += common_char as u32 - 96;
        } else {
            sum += common_char as u32 - 38;
        }
    }
    sum
}
