use std::fs;
// use  std::collections::BTreeMap
pub fn part1() {
    let file = fs::read_to_string("./day7p.txt").unwrap();
    let mut start_collecting = false;
    let mut all_sizes = vec![];
    let mut sum = 0;
    for line in file.lines() {
        if line.eq("$ ls") {
            start_collecting = true;
            continue;
        } else if line.starts_with("$") && start_collecting {
            println!("lll, {}", sum);

            if sum <= 100000 {
                all_sizes.push(sum);
            }
            sum = 0;

            start_collecting = false;
            continue;
        }
        if start_collecting {
            if !line.starts_with("dir") {
                let parts = line.split(" ").collect::<Vec<&str>>();
                // println!("PPP, {:#?}", parts[0].parse::<usize>().unwrap());

                sum += parts[0].parse::<usize>().unwrap();
            }
        }
    }
    let total: usize = all_sizes.iter().sum();
    println!("Here is final total: {}", total);
}
