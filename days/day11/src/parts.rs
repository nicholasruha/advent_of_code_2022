use num_bigint::BigUint;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::ops::{Add, Sub};

enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_str(string: &str) -> Self {
        match string {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Operation not found"),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    test_divisibility: u64,
    true_monkey: u64,
    false_monkey: u64,
    operation: String,
    operation_amount: String,
    inspected_count: u64,
}

impl Monkey {
    pub fn new(
        id: u64,
        items: &VecDeque<u64>,
        test_divisibility: u64,
        true_monkey: u64,
        false_monkey: u64,
        operation: &str,
        operation_amount: String,
    ) -> Self {
        Monkey {
            id,
            items: items.to_owned(),
            test_divisibility,
            true_monkey,
            false_monkey,
            operation: operation.to_owned(),
            operation_amount,
            inspected_count: 0,
        }
    }
}

pub fn part1() {
    // Correct 67830
    let file = fs::read_to_string("./days/day11/day11.txt").unwrap();

    let mut monkey_list = file
        .split("\r\n\r\n")
        .map(|monkey_block| {
            let lines = monkey_block.lines().collect::<Vec<&str>>();
            let monkey_id = lines[0].split(" ").collect::<Vec<&str>>()[1]
                .split(":")
                .collect::<Vec<_>>()[0]
                .parse::<u64>()
                .unwrap();

            let items = lines[1].split(":").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|item| item.trim_start().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let operation_instructions = lines[2].split(":").collect::<Vec<&str>>()[1]
                .split(" ")
                .collect::<Vec<_>>();
            let operation = operation_instructions[operation_instructions.len() - 2];
            let operation_amount = operation_instructions[operation_instructions.len() - 1];
            let test_divisibility = lines[3].split(" ").collect::<Vec<&str>>()[5]
                .parse::<u64>()
                .unwrap();
            let true_monkey = lines[4].split(" ").collect::<Vec<&str>>()[9]
                .parse::<u64>()
                .unwrap();
            let false_monkey = lines[5].split(" ").collect::<Vec<&str>>()[9]
                .parse::<u64>()
                .unwrap();
            Monkey::new(
                monkey_id,
                &VecDeque::from_iter(items),
                test_divisibility,
                true_monkey,
                false_monkey,
                operation,
                operation_amount.to_string(),
            )
        })
        .collect::<Vec<_>>();
    let magic_trick = monkey_list
        .iter()
        .map(|monkey| monkey.test_divisibility)
        .product::<u64>();

    // Do 20 rounds
    for _ in (0..20) {
        (0..monkey_list.len()).for_each(|i| {
            // Evaluate each item
            while let Some(worry_level) = monkey_list[i].items.pop_front() {
                let new_worry_level = compute_worry_level(
                    worry_level,
                    monkey_list[i].operation_amount.as_str(),
                    Op::from_str(monkey_list[i].operation.as_str()),
                    magic_trick,
                ) / 3;

                if new_worry_level % monkey_list[i].test_divisibility as u64 == 0 {
                    let true_monkey_id = monkey_list[i].true_monkey;
                    monkey_list[true_monkey_id as usize]
                        .items
                        .push_back(new_worry_level)
                } else {
                    let false_monkey_id = monkey_list[i].false_monkey;

                    monkey_list[false_monkey_id as usize]
                        .items
                        .push_back(new_worry_level)
                }
                monkey_list[i].inspected_count += 1;
            }
        })
    }
    monkey_list.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));
    let monkey_business = monkey_list[0].inspected_count * monkey_list[1].inspected_count;
    println!("{:#?}", monkey_business);
}
pub fn part2() {
    // Correct 15305381442
    let file = fs::read_to_string("./days/day11/day11.txt").unwrap();
    let mut monkey_map = HashMap::<u64, Monkey>::new();

    let mut monkey_list = file
        .split("\r\n\r\n")
        .map(|monkey_block| {
            let lines = monkey_block.lines().collect::<Vec<&str>>();
            let monkey_id = lines[0].split(" ").collect::<Vec<&str>>()[1]
                .split(":")
                .collect::<Vec<_>>()[0]
                .parse::<u64>()
                .unwrap();

            let items = lines[1].split(":").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|item| item.trim_start().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let operation_instructions = lines[2].split(":").collect::<Vec<&str>>()[1]
                .split(" ")
                .collect::<Vec<_>>();
            let operation = operation_instructions[operation_instructions.len() - 2];
            let operation_amount = operation_instructions[operation_instructions.len() - 1];
            let test_divisibility = lines[3].split(" ").collect::<Vec<&str>>()[5]
                .parse::<u64>()
                .unwrap();
            let true_monkey = lines[4].split(" ").collect::<Vec<&str>>()[9]
                .parse::<u64>()
                .unwrap();
            let false_monkey = lines[5].split(" ").collect::<Vec<&str>>()[9]
                .parse::<u64>()
                .unwrap();
            let monkey = Monkey::new(
                monkey_id,
                &VecDeque::from_iter(items),
                test_divisibility,
                true_monkey,
                false_monkey,
                operation,
                operation_amount.to_string(),
            );
            monkey_map.insert(monkey.id, monkey.clone());

            return monkey;
        })
        .collect::<Vec<_>>();
    let magic_trick = monkey_list
        .iter()
        .map(|monkey| monkey.test_divisibility)
        .product::<u64>();

    // Do 10000 rounds
    for _ in (0..10_000) {
        (0..monkey_list.len()).for_each(|i| {
            // Evaluate each item
            while let Some(worry_level) = monkey_list[i].items.pop_front() {
                let new_worry_level = compute_worry_level(
                    worry_level,
                    monkey_list[i].operation_amount.as_str(),
                    Op::from_str(monkey_list[i].operation.as_str()),
                    magic_trick,
                );

                if new_worry_level % monkey_list[i].test_divisibility as u64 == 0 {
                    let true_monkey_id = monkey_list[i].true_monkey;
                    monkey_list[true_monkey_id as usize]
                        .items
                        .push_back(new_worry_level)
                } else {
                    let false_monkey_id = monkey_list[i].false_monkey;

                    monkey_list[false_monkey_id as usize]
                        .items
                        .push_back(new_worry_level)
                }
                monkey_list[i].inspected_count += 1;
            }
        })
    }
    monkey_list.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));
    let monkey_business = monkey_list[0].inspected_count * monkey_list[1].inspected_count;
    println!("{:#?}", monkey_business);
    // println!("{:#?}", monkey_list);
}

fn compute_worry_level(worry_level: u64, amount: &str, operation: Op, magic_trick: u64) -> u64 {
    let mut evaluated_amount: u64 = 0;
    if amount.starts_with("old") {
        evaluated_amount = worry_level;
    } else {
        evaluated_amount = amount.parse::<u64>().unwrap();
    }

    match operation {
        Op::Add => (worry_level + evaluated_amount) % magic_trick,
        Op::Mul => (worry_level * evaluated_amount) % magic_trick,
    }
}
