use std::fs;

pub fn part1() -> u32 {
    let file = fs::read_to_string("./days/day1/day1.txt").unwrap();

    let result = file
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|food_item| food_item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result
}

pub fn part2() -> u32 {
    let file = fs::read_to_string("./days/day1/day1.txt").unwrap();

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
    final_result
}
