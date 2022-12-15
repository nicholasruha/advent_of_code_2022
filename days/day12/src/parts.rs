#![feature(iter_intersperse)]
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

fn priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => c as u32 - 'a' as u32 + 1,
        'S' => 0,
        'E' => 27,
        // c @ 'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Unacceptable character {c} !"),
    }
}

fn generate_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|row| row.chars().map(|char| priority(char)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) {
    let grid = generate_grid(input);
    let starting_pos = (0, 20);
    // println!("{:?}", grid);
    let height = grid.len();
    let width = grid[0].len();
    let cache_ = vec![vec![0; width]; height];

    (0..height).for_each(|row_pos| {
        (0..width).for_each(|col_pos| {
            get_climbable_neighbors(&grid, (row_pos, col_pos));
            //
        })
    })
}

fn get_climbable_neighbors(grid: Vec<Vec<u32>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let up = (position.0 - 1, position.1);
    let down = (position.0 + 1, position.1);
    let left = (position.0, position.1 - 1);
    let right = (position.0, position.1 + 1);

    let direction_position = vec![up, down, left, right];
    direction_position
        .into_iter()
        .filter(|modified_pos| is_neighbor_valid(&grid, position, modified_pos).is_some())
        .collect::<Vec<_>>()
}

fn is_neighbor_valid(
    grid: &Vec<Vec<u32>>,
    current_pos: (usize, usize),
    modified_pos: &(usize, usize),
) -> Option<()> {
    let current_value = &grid[current_pos.0][current_pos.1];
    let grid_pos = if let Some(row) = grid.get(modified_pos.0) {
        if let Some(col) = row.get(modified_pos.1) {
            let temp_pos = &grid[modified_pos.0][modified_pos.1];
            if (temp_pos - 1..=temp_pos + 1).contains(current_value) {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    grid_pos
}
