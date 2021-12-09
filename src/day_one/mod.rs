use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn read_file() -> Lines<BufReader<File>> {
    let file = File::open("src/day_one/task.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    return lines;
}

pub fn solve_part_one() {
    let lines = read_file();
    let mut last_value = i32::MAX;
    let mut result = 0;
    for line in lines {
        let current_value: i32 = line.unwrap().parse().unwrap();
        if last_value < current_value {
            result += 1;
        }

        last_value = current_value;
    }
    println!("${}", result);
}

struct SlidingWindow {
    value_one: i32,
    value_two: i32,
    value_three: i32,
}

impl SlidingWindow {
    fn new() -> SlidingWindow {
        return SlidingWindow {
            value_one: i32::MAX / 3,
            value_two: i32::MAX / 3,
            value_three: i32::MAX / 3,
        };
    }

    fn sum(&self) -> i32 {
        return self.value_one + self.value_two + self.value_three;
    }
}

pub fn solve_part_two() {
    let lines = read_file()
        .map(|line| line.unwrap().parse::<i32>())
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();

    let number_of_lines = lines.len();
    let mut result = 0;
    let mut last_sliding_window = SlidingWindow::new();
    for line_number in 0..number_of_lines {
        let current_sliding_window = SlidingWindow {
            value_one: lines[line_number],
            value_two: if line_number + 1 < number_of_lines { lines[line_number + 1] } else { 0 },
            value_three: if line_number + 2 < number_of_lines { lines[line_number + 2] } else { 0 },
        };

        if last_sliding_window.sum() < current_sliding_window.sum() {
            result += 1;
        }

        last_sliding_window = current_sliding_window;
    }

    println!("{}", result);
}
