use std::fs::File;
use std::io::{BufReader, Lines};

use crate::common::{read_file, separate_and_parse};

#[derive(Clone)]
struct Field {
    value: i32,
    checked: bool,
}

impl Field {
    fn new() -> Field {
        return Field {
            value: 0,
            checked: false,
        };
    }
}

#[derive(Clone)]
struct Board {
    size: usize,
    fields: Vec<Vec<Field>>,
    has_won: bool,
}

impl Board {
    fn new(size: usize) -> Board {
        let mut fields = Vec::new();
        for _ in 0..size {
            fields.push(vec!(Field::new(); size))
        }

        return Board {
            size,
            fields,
            has_won: false,
        };
    }

    fn set_rows(&mut self, rows: Vec<String>) {
        if rows.len() != self.size {
            panic!("Board should have size {} but has size {}", self.size, rows.len())
        }

        for (row, fields) in rows.into_iter().enumerate() {
            let fields = separate_and_parse::<i32>(fields, " ");
            for (column, field) in fields.into_iter().enumerate() {
                self.fields[row][column].value = field
            }
        }
    }

    fn check(&mut self, value: i32) {
        for row in 0..self.size {
            for column in 0..self.size {
                if self.fields[row][column].value == value {
                    self.fields[row][column].checked = true
                }
            }
        }
    }

    fn has_won(&mut self) -> bool {
        if self.has_won {
            return true;
        }

        let mut has_won = false;
        for row in 0..self.size {
            let mut all_checked = true;
            for column in 0..self.size {
                all_checked &= self.fields[row][column].checked
            }
            if all_checked {
                has_won = true
            }
        }
        for column in 0..self.size {
            let mut all_checked = true;
            for row in 0..self.size {
                all_checked &= self.fields[row][column].checked
            }
            if all_checked {
                has_won = true
            }
        }

        self.has_won = has_won;
        return has_won;
    }

    fn sum_unchecked_fields(&self) -> i32 {
        let mut sum = 0;
        for row in 0..self.size {
            for column in 0..self.size {
                let field = &self.fields[row][column];
                if !field.checked {
                    sum += field.value
                }
            }
        }

        return sum;
    }

    fn has_already_won_before(&self) -> bool {
        return self.has_won;
    }
}

pub fn solve_task_one(path: &str, size: usize) {
    let mut lines = read_file(path);
    let values = separate_and_parse::<i32>(lines.next().unwrap().unwrap(), ",");
    lines.next();
    let mut boards = build_boards(size, lines);
    for value in values {
        for board in &mut boards {
            board.check(value);
            if !board.has_already_won_before() && board.has_won() {
                println!("Solution: {}", board.sum_unchecked_fields() * value);
            }
        }
    }
}

fn build_boards(size: usize, lines: Lines<BufReader<File>>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut rows: Vec<String> = Vec::new();
    for line in lines {
        let line = line.expect("");
        if line.is_empty() {
            add_board(size, &mut boards, rows);
            rows = Vec::new();
        } else {
            rows.push(line)
        }
    }

    add_board(size, &mut boards, rows);
    return boards;
}

fn add_board(size: usize, boards: &mut Vec<Board>, rows: Vec<String>) {
    let mut board = Board::new(size);
    board.set_rows(rows);
    boards.push(board);
}