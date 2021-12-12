use std::fmt::format;

use crate::common::read_file;

pub fn solve_task_one() {
    let values = read_file("src/day_three/task.txt")
        .map(|line| usize::from_str_radix(&*line.unwrap(), 2).unwrap())
        .collect::<Vec<usize>>();

    let number_of_bits = 12;
    let max_shift = number_of_bits - 1;
    let number_of_values = values.len();
    let mut occurrences = vec![0; number_of_bits];
    values.iter().for_each(|value| {
        for index in 0..number_of_bits {
            let result = (*value & (1 << max_shift - index)) >> (max_shift - index);
            occurrences[index] += result
        }
    });
    let mut gamma_str = "".to_owned();
    occurrences.iter().for_each(|occurrence| {
        gamma_str.push_str(evaluate_occurrence(number_of_values, *occurrence))
    });

    let gamma = usize::from_str_radix(&*gamma_str, 2).unwrap();
    let x = (2_i32.pow(number_of_bits as u32)) - 1;
    let epsilon = gamma ^ (x as usize);
    println!("Solution: {}", gamma * epsilon)
}

fn evaluate_occurrence(number_of_values: usize, occurrence: usize) -> &'static str {
    if occurrence > (number_of_values / 2) {
        "1"
    } else {
        "0"
    }
}

struct Value {
    bits: Vec<usize>,
}

impl Value {
    fn new(line: String) -> Value {
        let bits = line.chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        return Value { bits };
    }

    fn get(&self, position: usize) -> usize {
        return self.bits[position];
    }

    fn size(&self) -> usize {
        return self.bits.len();
    }

    fn to_usize(&self) -> usize {
        let mut str = "".to_owned();
        self.bits.iter().for_each(|bit| str.push_str(&*bit.to_string()));
        return usize::from_str_radix(&*str, 2).unwrap();
    }
}

pub fn solve_task_two() {
    let oxygen_generator_rating = calculate_rating(
        &(|value: &Value, index: usize| value.get(index) == 1),
        &(|value: &Value, index: usize| value.get(index) == 0)
    );
    let co2_scrubber_rating = calculate_rating(
        &(|value: &Value, index: usize| value.get(index) == 0),
        &(|value: &Value, index: usize| value.get(index) == 1)
    );
    println!("Solution: {}", oxygen_generator_rating* co2_scrubber_rating);
}

fn calculate_rating<F1: Fn(&Value, usize) -> bool, F2: Fn(&Value, usize) -> bool>(more_ones: &F1, more_zeroes: &F2) -> usize {
    let mut values = read_file("src/day_three/task.txt")
        .map(|line| Value::new(line.unwrap()))
        .collect::<Vec<Value>>();

    let size = values[0].size();
    for index in 0..size {
        if values.len() == 1 {
            break;
        }

        let number_of_ones = values.iter().fold(0, |acc, value| acc + value.get(index));
        if number_of_ones * 2 >= values.len() {
            values.retain(|value| more_ones(value, index))
        } else {
            values.retain(|value| more_zeroes(value, index))
        }
    }
    return values[0].to_usize();
}