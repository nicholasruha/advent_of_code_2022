#![feature(iter_array_chunks)]

mod parts;

fn main() {
    let part1_result = parts::part1();
    let part2_result = parts::part2();
    assert_eq!(part1_result, "");
    assert_eq!(part2_result, "");
}
