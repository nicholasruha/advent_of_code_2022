use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn elf_with_most_calories_1() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let result = &file
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|food_item| food_item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{}", result);
}
pub fn elf_with_most_calories_2() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let mut result = file
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|food_item| food_item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let final_result: u32 = result.iter().take(3).sum();
    println!("{}", final_result);
}

pub fn elf_with_most_calories() -> Vec<usize> {
    let mut highest = vec![0; 3];
    let mut all = vec![];
    if let Ok(lines) = read_lines("./day1.txt") {
        let mut current = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    let min_value = highest.iter().fold(0, |mut min, &val| {
                        if val <= min {
                            min = val;
                        }
                        min
                    });
                    let index = highest.iter().position(|&r| r == min_value);

                    if index.is_some() && min_value < current {
                        highest[index.unwrap()] = current;
                    }
                    all.push(current);
                    println!("{}", ip);
                    current = 0;
                } else {
                    let food_item = ip.parse::<usize>().unwrap();
                    current += food_item;
                    println!("{}", ip);
                }
            }
        }
    }
    all.sort();
    println!("{:?}", all);
    highest
    // [71023, 67232, 65687] -> 203942  wrong
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
