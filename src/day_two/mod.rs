use std::ops::Sub;

use crate::common::read_file;

pub fn solve_task_one() {
    struct TaskOneSubmarine {
        horizontal_position: i32,
        vertical_position: i32,
    }

    impl Submarine for TaskOneSubmarine {
        fn forward(&mut self, unit: i32) { self.horizontal_position += unit }
        fn down(&mut self, unit: i32) { self.vertical_position += unit }
        fn up(&mut self, unit: i32) { self.vertical_position -= unit }
    }

    let mut submarine = TaskOneSubmarine {
        horizontal_position: 0,
        vertical_position: 0
    };

    drive_submarine(&mut submarine);
    println!("Solution Task One: {}", submarine.horizontal_position * submarine.vertical_position)
}

pub fn solve_task_two() {
    struct TaskTwoSubmarine {
        horizontal_position: i32,
        vertical_position: i32,
        aim: i32,
    }

    impl Submarine for TaskTwoSubmarine {
        fn forward(&mut self, unit: i32) {
            self.horizontal_position += unit;
            self.vertical_position += self.aim * unit;
        }

        fn down(&mut self, unit: i32) {
            self.aim += unit
        }

        fn up(&mut self, unit: i32) {
            self.aim -= unit
        }
    }

    let mut submarine = TaskTwoSubmarine {
        horizontal_position: 0,
        vertical_position: 0,
        aim: 0,
    };

    drive_submarine(&mut submarine);
    println!("Solution Task Two: {}", submarine.horizontal_position * submarine.vertical_position)
}

trait Submarine {
    fn forward(&mut self, unit: i32);
    fn down(&mut self, unit: i32);
    fn up(&mut self, unit: i32);
}

fn drive_submarine(submarine: &mut impl Submarine) {
    let lines = read_file("src/day_two/task.txt");
    let lines = lines.filter_map(|line| line.ok()).collect::<Vec<String>>();
    for line in lines {
        let parts = line.split(" ").map(|part| part.trim()).collect::<Vec<&str>>();
        let direction = parts[0];
        let unit = parts[1].parse::<i32>().unwrap();
        match direction {
            "forward" => submarine.forward(unit),
            "down" => submarine.down(unit),
            "up" => submarine.up(unit),
            _ => panic!("Unknown direction {}", direction)
        }
    }
}