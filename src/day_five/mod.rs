use std::cmp::{max, min};

use crate::common::{read_file, separate, separate_and_parse};

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        return Point { x, y };
    }

    fn distance_from_origin(&self) -> f64 {
        return ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt();
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        if p1.distance_from_origin() < p2.distance_from_origin() {
            return Line { p1, p2 };
        }

        return Line { p1: p2, p2: p1 };
    }

    fn max_coordinate(&self) -> usize {
        let max_x = max(self.p1.x, self.p2.x);
        let max_y = max(self.p1.y, self.p2.y);
        return max(max_x, max_y);
    }

    fn vertical(&self) -> bool {
        return self.p1.x == self.p2.x && !self.horizontal();
    }

    fn horizontal(&self) -> bool {
        return self.p1.y == self.p2.y && !self.vertical();
    }

    fn diagonal(&self) -> bool {
        let diff_x = isize::abs((self.p1.x as isize) - (self.p2.x as isize));
        let diff_y = isize::abs((self.p1.y as isize) - (self.p2.y as isize));
        return diff_x == diff_y;
    }

    fn diagonally_downwards(&self) -> bool {
        return self.p1.y < self.p2.y && self.p1.x < self.p2.x;
    }

    fn start_x(&self) -> usize {
        return min(self.p1.x, self.p2.x);
    }

    fn end_x(&self) -> usize {
        return max(self.p1.x, self.p2.x);
    }

    fn start_y(&self) -> usize {
        return min(self.p1.y, self.p2.y);
    }

    fn end_y(&self) -> usize {
        return max(self.p1.y, self.p2.y);
    }
}

struct Diagram {
    fields: Vec<Vec<u32>>,
}

impl Diagram {
    fn new(size: usize) -> Diagram {
        let mut fields = Vec::new();
        for _ in 0..size {
            fields.push(vec!(0; size))
        }

        return Diagram {
            fields
        };
    }

    fn draw(&mut self, line: &Line) {
        if line.horizontal() || line.vertical() {
            self.draw_horizontal_and_vertical(line)
        } else {
            self.draw_diagonal(line)
        }
    }

    fn draw_horizontal_and_vertical(&mut self, line: &Line) {
        for x in line.start_x()..line.end_x() + 1 {
            for y in line.start_y()..line.end_y() + 1 {
                self.fields[x as usize][y as usize] += 1
            }
        }
    }

    fn draw_diagonal(&mut self, line: &Line) {
        if line.diagonally_downwards() {
            for pos in 0..(line.end_y() - line.start_y()) + 1 {
                let x = line.start_x() + pos;
                let y = line.start_y() + pos;
                self.fields[x][y] += 1
            }
        } else {
            for pos in 0..(line.end_y() - line.start_y()) + 1 {
                let x = line.start_x() + pos;
                let y = line.end_y() - pos;
                self.fields[x][y] += 1
            }
        }
    }

    fn count_cross_points(&self) -> u32 {
        let limit: u32 = 1;
        let num_cross_points = self.fields
            .iter()
            .map(|row| row
                .into_iter()
                .filter(|x| **x > limit)
                .count() as u32)
            .sum::<u32>();

        return num_cross_points;
    }
}

fn calculate_size_of_diagram(lines: &Vec<Line>) -> usize {
    lines.iter().map(|line| line.max_coordinate()).max().unwrap() as usize + 1
}

fn build_point(coordinates_str: &str) -> Point {
    let coordinates = separate_and_parse::<usize>(coordinates_str.to_string(), ",");
    Point::new(coordinates[0], coordinates[1])
}

fn solve(path: &str) {
    let file_lines = read_file(path);
    let lines = file_lines
        .map(|line| line.expect(""))
        .map(|line| {
            let points_str = separate(line, "->");
            let point1 = build_point(&points_str[0]);
            let point2 = build_point(&points_str[1]);
            Line::new(point1, point2)
        })
        .collect::<Vec<Line>>();

    let size = calculate_size_of_diagram(&lines);
    let mut diagram = Diagram::new(size);
    for line in &lines {
        diagram.draw(line)
    }

    println!("Solution: {}", diagram.count_cross_points());
}

pub fn solve_task_one(path: &str) {
    solve(path)
}

pub fn solve_task_two(path: &str) {
    solve(path)
}
