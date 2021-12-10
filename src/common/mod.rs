use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    return lines;
}
