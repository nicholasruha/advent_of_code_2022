use std::{collections::HashMap, fs};

pub fn part1() -> usize {
    // 1538 is correct
    let file = fs::read_to_string("./days/day8/day8.txt").unwrap();
    let tree_grid = file
        .lines()
        .map(|row| {
            row.chars()
                .map(|tree_height| {
                    let tree_height = tree_height.to_digit(10).unwrap();
                    tree_height
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let height = tree_grid.len();
    let width = tree_grid[0].len();
    let mut total_visible = 0;

    for (row_pos, row) in tree_grid.iter().enumerate() {
        if row_pos == 0 || row_pos == height - 1 {
            continue;
        }
        'column: for (col_pos, tree_height) in row.iter().enumerate() {
            if col_pos == 0 || col_pos == width - 1 {
                continue;
            }

            // Check if trees to the left are visible
            let is_left = (0..col_pos)
                .rev()
                .all(|i| tree_grid[row_pos][i] < tree_grid[row_pos][col_pos]);

            // Right
            let is_right =
                (col_pos + 1..width).all(|i| tree_grid[row_pos][i] < tree_grid[row_pos][col_pos]);

            // Check if trees above are visible
            let is_above = (0..row_pos)
                .rev()
                .all(|i| tree_grid[i][col_pos] < tree_grid[row_pos][col_pos]);

            let is_below =
                (row_pos + 1..height).all(|i| tree_grid[i][col_pos] < tree_grid[row_pos][col_pos]);

            if is_right || is_left || is_above || is_below {
                total_visible += 1;
            }
        }
    }
    let parameter = 2 * (height - 1) + 2 * (width - 1);

    total_visible + parameter
}

pub fn part2() -> i32 {
    // Correct 496125
    let file = fs::read_to_string("./days/day8/day8.txt").unwrap();
    let tree_grid = file
        .lines()
        .map(|row| {
            row.chars()
                .map(|tree_height| {
                    let tree_height = tree_height.to_digit(10).unwrap();
                    tree_height
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    let position_map = HashMap::<(usize, usize), usize>::new();
    let height = tree_grid.len();
    let width = tree_grid[0].len();

    // println!("{:#?}", tree_grid);

    let best_view_score = tree_grid
        .iter()
        .enumerate()
        .filter_map(|(row_pos, row)| {
            if !(row_pos == 0 || row_pos == height - 1) {
                let highest_product = row
                    .iter()
                    .enumerate()
                    .filter_map(|(col_pos, tree_height)| {
                        if !(col_pos == 0 || col_pos == width - 1) {
                            // Left
                            let mut left_value = 0;
                            let mut right_value = 0;
                            let mut above_value = 0;
                            let mut below_value = 0;
                            for i in (0..col_pos).rev() {
                                left_value += 1;
                                if tree_grid[row_pos][i] >= tree_grid[row_pos][col_pos] {
                                    break;
                                }
                            }
                            for i in (col_pos + 1..width) {
                                right_value += 1;
                                if tree_grid[row_pos][i] >= tree_grid[row_pos][col_pos] {
                                    break;
                                }
                            }
                            for i in (0..row_pos).rev() {
                                above_value += 1;
                                if tree_grid[i][col_pos] >= tree_grid[row_pos][col_pos] {
                                    break;
                                }
                            }
                            for i in row_pos + 1..height {
                                below_value += 1;
                                if tree_grid[i][col_pos] >= tree_grid[row_pos][col_pos] {
                                    break;
                                }
                            }

                            println!(" {} {} {} {}", left_value, right_value, above_value, below_value);
                            // println!("{}", tree_grid[row_pos][col_pos]);
                            let tt = left_value * right_value * above_value * below_value;
                            return Some(tt);
                        }
                        None
                    }).max(); // .collect::<Vec<_>>();
                return Some(highest_product);
            }
            None
        }).max();
        // .collect::<Vec<_>>();

    println!("Here is the best score {:#?} ", best_view_score.unwrap());
    best_view_score.unwrap().unwrap()
}
