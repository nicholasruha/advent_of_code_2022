use std::collections::VecDeque;
use std::fs;

fn create_execution_stack() -> VecDeque<i32> {
    let file = fs::read_to_string("./days/day10/day10.txt").unwrap();

    let mut execution_stack = VecDeque::new();

    file.lines().for_each(|instruction| {
        if instruction.starts_with("noop") {
            execution_stack.push_front(0);
        } else {
            let amount = instruction.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            execution_stack.push_front(0);
            execution_stack.push_front(amount);
        }
    });
    execution_stack
}

pub fn part1() {
    // correct 14720
    let mut execution_stack = create_execution_stack();

    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut total = 0;

    while let Some(amount) = execution_stack.pop_back() {
        cycle_count += 1;
        if cycle_count == 20
            || cycle_count == 60
            || cycle_count == 100
            || cycle_count == 140
            || cycle_count == 180
            || cycle_count == 220
        {
            total += cycle_count * register_value;
            println!("Cycle {} has a total of {}", cycle_count, total);
        }
        register_value += amount;
    }

    println!("total, {}", total);
}

pub fn part2() {
    let mut execution_stack = create_execution_stack();

    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut cpu_visual_output = String::from("");
    while let Some(amount) = execution_stack.pop_back() {
        cycle_count += 1;
        cycle_count = cycle_count % 40;
        register_value += amount;
        println!("{} {} {}", cycle_count, register_value, (cycle_count - 1..=cycle_count + 1).contains(&register_value));

        if (cycle_count - 1..=cycle_count + 1).contains(&register_value) {
            cpu_visual_output.push('#'); // █
        } else {
            cpu_visual_output.push('.');
        }
        // register_value += amount;
    }

    cpu_visual_output
        .chars()
        .enumerate()
        .for_each(|(index, letter)| {
            if index % 40 == 0 {
                println!("");
            }
            print!("{}", letter);
        })
}
