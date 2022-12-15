use std::fs;

mod parts;

fn main() {
    let input = fs::read_to_string("./days/day7/day7.txt").unwrap();

    let part1_result = parts::part1(&input);
    // let part2_result = parts::part2(&input);
    println!("{} ", part1_result);
    // println!("{} ", part2_result);
}
