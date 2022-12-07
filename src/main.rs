#![feature(iter_array_chunks)]

use day1::elf_with_most_calories;
use day2::{rps_calculator, rps_calculator_part_2};
use std::{
    fs,
    ops::{Range, RangeInclusive},
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod file;

fn main() {
    let a = day7::part1();
    let y = [1, 2, 3];
    // y.windows(size)
    // y.array_windows(size)
}
