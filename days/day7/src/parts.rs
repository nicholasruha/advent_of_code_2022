use std::fs;

pub fn part1() {
    let file = fs::read_to_string("./days/day7/day7.txt").unwrap();

    // let path = vec![];

    file.split("\n$").collect::<Vec<_>>().iter().skip(1).for_each(|block| {
        let command_info = block.split("\n").collect::<Vec<_>>();
        let command = &command_info[0];
        let outputs = &command_info[1..];

        let parts = command.split(" ").collect::<Vec<_>>();
        let op = parts[0];
    });
    
    // println!("Here is final total: {}", total);
}
