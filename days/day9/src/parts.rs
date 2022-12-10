use std::{collections::HashSet, fs};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid String"),
        }
    }
}

fn where_tail_should_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // Should it move diagonally?
    if head.0 != tail.0 && head.1 != tail.1 {
        let x_sign = head.0 - tail.0;
        let y_sign = head.1 - tail.1;

        if x_sign > 1 {
            // Will move to the Right
            if y_sign > 0 {
                // Will move Up
                return (1, 1);
            } else {
                //if y_sign < 0 {
                // Will move Down
                return (1, -1);
            }
        } else if x_sign < -1 {
            // Will move to the Left
            if y_sign > 0 {
                // Will move Up
                return (-1, 1);
            } else {
                //if y_sign < 0 {
                // Will move Down
                return (-1, -1);
            }
        } else if y_sign > 1 {
            // Move Up
            if x_sign > 0 {
                // Move Right
                return (1, 1);
            } else {
                return (-1, 1);
            }
        } else if y_sign < -1 {
            // Move Down
            if x_sign > 0 {
                // Move Right
                return (1, -1);
            } else {
                return (-1, -1);
            }
        } else {
            return (0, 0);
        }
    }
    // Should if move horizontally
    else if head.0 != tail.0 {
        let sign = head.0 - tail.0;
        if sign < -1 {
            // Move to the left
            return (-1, 0);
        } else if sign > 1 {
            // Move to the right
            return (1, 0);
        } else {
            // Dont move
            return (0, 0);
        }
        // Maybe move vertically
    } else if head.1 != tail.1 {
        let sign = head.1 - tail.1;
        if sign < -1 {
            // Move Down
            return (0, -1);
        } else if sign > 1 {
            // Move Up
            return (0, 1);
        } else {
            // Dont move
            return (0, 0);
        }
    } else {
        return (0, 0);
    }
}
fn where_tail_should_move2(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0 != tail.0 && head.1 != tail.1 {
        // It should move diagonally
        let x_sign = head.0 - tail.0;
        let y_sign = head.1 - tail.1;
        if x_sign > 1 {
            // Will move to the Right
            if y_sign > 0 {
                // Will move Up
                return (1, 1);
            } else if y_sign < 0 {
                //if y_sign < 0 {
                // Will move Down
                return (1, -1);
            } else {
                return (0, 0);
            }
        } else if x_sign < -1 {
            // Will move to the Left
            if y_sign > 0 {
                // Will move Up
                return (-1, 1);
            } else if y_sign < 0 {
                //if y_sign < 0 {
                // Will move Down
                return (-1, -1);
            } else {
                return (0, 0);
            }
        } else if y_sign > 1 {
            // Move Up
            if x_sign > 0 {
                // Move Right
                return (1, 1);
            } else if x_sign < 0 {
                return (-1, 1);
            } else {
                return (0, 0);
            }
        } else if y_sign < -1 {
            // Move Down
            if x_sign > 0 {
                // Move Right
                return (1, -1);
            } else if x_sign < 0 {
                return (-1, -1);
            } else {
                return (0, 0);
            }
        } else {
            return (0, 0);
        }
    }
    // Should if move horizontally
    else if head.0 != tail.0 {
        let sign = head.0 - tail.0;
        if sign < -1 {
            // Move to the left
            return (-1, 0);
        } else if sign > 1 {
            // Move to the right
            return (1, 0);
        } else {
            // Dont move
            return (0, 0);
        }
        // Maybe move vertically
    } else if head.1 != tail.1 {
        let sign = head.1 - tail.1;
        if sign < -1 {
            // Move Down
            return (0, -1);
        } else if sign > 1 {
            // Move Up
            return (0, 1);
        } else {
            // Dont move
            return (0, 0);
        }
    } else {
        return (0, 0);
    }
}

pub fn part1() -> usize{
    // Correct: 6337
    let file = fs::read_to_string("./days/day9/day9.txt").unwrap();
    let mut previous_positions = HashSet::<(i32, i32)>::new();
    previous_positions.insert((0, 0));
    let mut head = (0, 0);
    let mut tail = (0, 0);
    file.lines().for_each(|instruction| {
        let single_instruction = instruction.split(" ").collect::<Vec<_>>();
        let direction = Direction::from_str(single_instruction[0]);
        let amount = single_instruction[1].parse::<u32>().unwrap();
        for _ in 0..amount {
            match direction {
                Direction::Up => {
                    head.1 += 1;
                    let move_coords = where_tail_should_move(head, tail);
                    tail.0 += move_coords.0;
                    tail.1 += move_coords.1;
                    if !previous_positions.contains(&tail) {
                        previous_positions.insert(tail);
                    }
                }
                Direction::Down => {
                    head.1 += -1;
                    let move_coords = where_tail_should_move(head, tail);
                    tail.0 += move_coords.0;
                    tail.1 += move_coords.1;
                    if !previous_positions.contains(&tail) {
                        previous_positions.insert(tail);
                    }
                }
                Direction::Right => {
                    head.0 += 1;
                    let move_coords = where_tail_should_move(head, tail);
                    tail.0 += move_coords.0;
                    tail.1 += move_coords.1;
                    if !previous_positions.contains(&tail) {
                        previous_positions.insert(tail);
                    }
                }
                Direction::Left => {
                    head.0 += -1;
                    let move_coords = where_tail_should_move(head, tail);
                    tail.0 += move_coords.0;
                    tail.1 += move_coords.1;
                    if !previous_positions.contains(&tail) {
                        previous_positions.insert(tail);
                    }
                }
            }
        }
    });
    println!(
        "Total Number of unique positions {}",
        previous_positions.len()
    );
    previous_positions.len()
}
pub fn part2() -> usize {
    // Correct:
    let file = fs::read_to_string("./days/day9/day9p2.txt").unwrap();
    let mut previous_positions = HashSet::<(i32, i32)>::from_iter(vec![(0, 0)]);
    let mut rope = vec![(0, 0); 9];
    let rope_head_pos = rope.len() - 1;
    let rope_tail_pos = 0;
    let mut c = 0;

    file.lines().for_each(|instruction| {
        let single_instruction = instruction.split(" ").collect::<Vec<_>>();
        let direction = Direction::from_str(single_instruction[0]);
        let amount = single_instruction[1].parse::<u32>().unwrap();
        for _ in 0..amount {
            match direction {
                Direction::Up => {
                    rope[rope_head_pos].1 += 1;
                }
                Direction::Down => {
                    rope[rope_head_pos].1 += -1;
                }
                Direction::Right => {
                    rope[rope_head_pos].0 += 1;
                }
                Direction::Left => {
                    rope[rope_head_pos].0 += -1;
                }
            }
            (1..=rope_head_pos).rev().for_each(|index| {
                let move_coords = where_tail_should_move2(rope[index], rope[index - 1]);
                rope[index - 1].0 += move_coords.0;
                rope[index - 1].1 += move_coords.1;
            });
            if !previous_positions.contains(&rope[0]) {
                previous_positions.insert(rope[0]);
            }
            c += 1;
        }
        // if c > 19 {
        //     panic!("OKOKOKOK");
        // }
    });
    println!("{:#?}", previous_positions);
    println!(
        "Total Number of unique positions {}",
        previous_positions.len()
    );
    previous_positions.len()
}
// 2674 incorrect, too high
