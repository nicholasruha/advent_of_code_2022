use std::fs;

// Note I had to cheat a little. I added an extra space in the txt file for each line because array_chunks will only take n chucks, so if you have n-1 chunks remaining, then it will stop prematurely.
pub fn part1() {
    let (mut crates, instructions) = parse_into_crates_and_instructions();
    println!("{:#?}, {}", crates, crates.len());
    // panic!("ADF");
    instructions.iter().for_each(|instruction| {
        let instruction_parts = instruction.split("from").collect::<Vec<_>>();
        let amount = instruction_parts[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        if let [from, to] = instruction_parts[1].split("to").collect::<Vec<_>>()[0..2] {
            let f = from.trim().parse::<usize>().unwrap() - 1;
            let t = to.trim().parse::<usize>().unwrap() - 1;
            part1_move_crate_procedure(&mut crates, f, t, amount);
        }
    });
    println!("{:#?}", crates);
    get_tops(&mut crates);
}

pub fn part2() {
    let (mut crates, instructions) = parse_into_crates_and_instructions();
    // println!("{:#?}, {}", crates, crates.len());
    // panic!("ADF");
    instructions.iter().for_each(|instruction| {
        let instruction_parts = instruction.split("from").collect::<Vec<_>>();
        let amount = instruction_parts[0].split(" ").collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        if let [from, to] = instruction_parts[1].split("to").collect::<Vec<_>>()[0..2] {
            let f = from.trim().parse::<usize>().unwrap() - 1;
            let t = to.trim().parse::<usize>().unwrap() - 1;
            part2_move_crate_procedure(&mut crates, f, t, amount);
        }
    });
    println!("{:#?}", crates);
    get_tops(&mut crates);
}

// Incorrect PVJCFZTQS, BVGCPTTBR, JSCZSJVRT
// Correct!: SHMSDGZVC

fn part2_move_crate_procedure(
    crates: &mut Vec<Vec<String>>,
    from: usize,
    to: usize,
    amount: usize,
) {
    let offset = crates[from].len() - amount;
    let pp = crates[from].drain(offset..).collect::<Vec<String>>();
    crates[to].extend(pp);
} // Correct! : VRZGHDFBQ

fn part1_move_crate_procedure(
    crates: &mut Vec<Vec<String>>,
    from: usize,
    to: usize,
    amount: usize,
) {
    for _ in 0..amount {
        let popped = crates[from].pop();
        if popped.is_some() {
            crates[to].push(popped.unwrap());
        }
    }
}

fn get_tops(crates: &mut Vec<Vec<String>>) {
    crates.iter_mut().for_each(|l| {
        println!("{}", l.pop().unwrap());
    })
}

fn compute_crate_rows<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Vec<String>> {
    let mut all_crates = lines
        .map(|crate_row| {
            print!("{} ", crate_row);
            let o = crate_row
                .bytes()
                .map(|single_char| single_char as char)
                .array_chunks::<4>()
                .map(|char_chucks| char_chucks.map(|m| m.to_string()).join(" "))
                .collect::<Vec<String>>();
            println!("{:#?} ", o);
            o
        })
        .collect::<Vec<Vec<String>>>();
    all_crates.reverse();
    all_crates
}

fn parse_into_crates_and_instructions() -> (Vec<Vec<String>>, Vec<String>) {
    let file = fs::read_to_string("./day5.txt").unwrap();

    let crate_lines = file.lines().take(8);
    let crates = compute_crate_rows(crate_lines);

    let instructions = file
        .lines()
        .skip(10)
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    (turn_crate_rows_to_columns(&crates), instructions)
}

fn turn_crate_rows_to_columns(rows: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    println!("{:#?}, {}", rows, rows.len());
    let m = rows
        .iter()
        .fold(vec![Vec::<String>::new(); rows.len() + 1], |mut acc, xs| {
            for (index, x) in xs.iter().enumerate() {
                println!("{}", x);

                if !x.trim().is_empty() {
                    acc[index].push(x.to_owned());
                }
            }
            acc
        });
    println!("{:#?}", m);
    m
}
