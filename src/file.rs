use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Read};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// pub struct ToLine<R> {
//     reader: R,
// }

// impl <R:Read> Iterator for ToLine<R> {
//     type Item = io::Result<Vec<String>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.reader.
//     }
// }
