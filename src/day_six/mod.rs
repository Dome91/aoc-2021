use crate::common::{read_file, separate, separate_and_parse};

struct Lanternfish {
    internal_timer: usize,
}

impl Lanternfish {
    fn new(internal_timer: usize) -> Lanternfish {
        return Lanternfish {
            internal_timer
        };
    }

    fn day_has_passed(&mut self) -> Option<Lanternfish> {
        if self.internal_timer == 0 {
            self.internal_timer = 6;
            return Option::Some(Lanternfish::new(8));
        }

        self.internal_timer -= 1;
        return Option::None;
    }
}

pub fn solve_task_one(path: &str, number_of_days: usize) {
    let mut lines = read_file(path);
    let initial_internal_timers = separate_and_parse::<usize>(lines.next().unwrap().unwrap(), ",");
    let mut lanternfishs = initial_internal_timers
        .into_iter()
        .map(|internal_timer| Lanternfish::new(internal_timer))
        .collect::<Vec<Lanternfish>>();

    for day in 0..number_of_days {
        let mut new_lanternfishs = lanternfishs
            .iter_mut()
            .map(|lanternfish| lanternfish.day_has_passed())
            .filter(|new_lanternfish_option| new_lanternfish_option.is_some())
            .map(|new_lantern_fish| new_lantern_fish.unwrap())
            .collect::<Vec<Lanternfish>>();

        lanternfishs.append(&mut new_lanternfishs);
        println!("Day {}", day)
    }

    println!("Solution: {}", lanternfishs.len())
}

pub fn solve_task_two(path: &str, number_of_days: usize) {
    let mut lines = read_file(path);
    let initial_internal_timers = separate_and_parse::<usize>(lines.next().unwrap().unwrap(), ",");
    let mut lanternfish: [usize; 10] = [0; 10];
    initial_internal_timers.into_iter().for_each(|timer| {
        lanternfish[timer] += 1;
    });

    for _ in 0..number_of_days {
        lanternfish[7] += lanternfish[0];
        lanternfish[9] = lanternfish[0];
        for index in 0..9 {
            lanternfish[index] = lanternfish[index+1];
        }
        lanternfish[9] = 0;
    }

    println!("Solution: {}", lanternfish.iter().sum::<usize>());
}