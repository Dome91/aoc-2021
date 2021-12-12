use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

pub fn read_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    return lines;
}

pub fn separate_and_parse<ITEM>(string: String, separator: &str) -> Vec<ITEM> where
    ITEM: FromStr,
    <ITEM as FromStr>::Err: fmt::Debug
{
    return string
        .split(separator)
        .into_iter()
        .filter(|value| !value.is_empty())
        .map(|value| value.trim())
        .map(|value| value.parse::<ITEM>().expect("Cannot parse value"))
        .collect::<Vec<ITEM>>();
}