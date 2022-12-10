// 9-86,9-87
// 9-87,9-86
// 10-86,9-86

use std::fs;

pub fn part1() -> usize {
    let file = fs::read_to_string("./days/day4/day4.txt").unwrap();
    let result = file
        .lines()
        .map(|range_team| {
            let elf_ranges = range_team.split(",").collect::<Vec<&str>>();
            let elf1 = elf_ranges[0]
                .to_string()
                .split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let elf2 = elf_ranges[1]
                .to_string()
                .split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            println!("{:?}, {:?}", elf1, elf2);
            if elf1[0] >= elf2[0] && elf1[1] <= elf2[1] || elf1[0] <= elf2[0] && elf1[1] >= elf2[1]
            {
                return 1;
            }

            0
        })
        .sum::<usize>();
    result
}

fn is_in_range(range: &(usize, usize), num: usize) -> bool {
    (range.0..=range.1).contains(&num)
}
pub fn part2() -> usize {
    // Not correct, the rigth anwser was:
    let file = fs::read_to_string("./days/day4/day4.txt").unwrap();
    // let mut list_of_ranges: Vec<(usize, usize)> = vec![(2, 4), (6, 8)];  // For practice list
    let mut list_of_ranges: Vec<(usize, usize)> = vec![(18, 20), (19, 21)];
    let result = file
        .lines()
        .map(|range_team| {
            let elf_ranges = range_team.split(",").collect::<Vec<&str>>();
            // let elf_ranges = range_team.split(",").collect::<Vec<&str>>();

            let elf1 = elf_ranges[0]
                .to_string()
                .split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let elf2 = elf_ranges[1]
                .to_string()
                .split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let is_in_previous_range = list_of_ranges
                .iter()
                .any(|previous_range| is_in_range(previous_range, elf1[0]));
            let is_in_previous_range2 = list_of_ranges
                .iter()
                .any(|previous_range| is_in_range(previous_range, elf1[1]));
            let is_in_previous_range3 = list_of_ranges
                .iter()
                .any(|previous_range| is_in_range(previous_range, elf2[0]));
            let is_in_previous_range4 = list_of_ranges
                .iter()
                .any(|previous_range| is_in_range(previous_range, elf2[1]));

            println!(
                "{:?}, {:?}, {:?}, {:?}",
                is_in_previous_range,
                is_in_previous_range2,
                is_in_previous_range3,
                is_in_previous_range4
            );
            if is_in_previous_range
                && is_in_previous_range2
                && is_in_previous_range3
                && is_in_previous_range4
            {
                return 1;
            } else {
                list_of_ranges.push((elf1[0], elf1[1]));
                list_of_ranges.push((elf2[0], elf2[1]));
                return 0;
            }
        })
        .sum::<usize>();
    result
}
