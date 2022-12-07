use std::collections::VecDeque;
use std::{collections::HashSet, fs};

pub fn part1() {
    // Correct: 1623
    let file = fs::read_to_string("./day6.txt").unwrap();
    file.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<HashSet<&char>>();
            println!("as index: {}", _i + 4);
            slice.len() == set.len()
        });
}
pub fn part2() {
    // Correct 3774
    let file = fs::read_to_string("./day6.txt").unwrap();
    file.chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<HashSet<&char>>();
            println!("as index: {}", _i + 14);
            slice.len() == set.len()
        });
}
