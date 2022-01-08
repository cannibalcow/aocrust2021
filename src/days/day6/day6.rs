use colored::Colorize;

use crate::tools::{self, pretty_print_result};

pub fn solve_part1() {
    tools::pretty_print_day(6, 1);
    let input = parse_file("D06_input.txt");
    let fishes = input_to_fishes(&input);

    let mut state = State::new(fishes);

    for _ in 0..80 {
        state.next_day();
    }
    pretty_print_result(state.count());
}

pub fn solve_part2() {
    tools::pretty_print_day(6, 2);
    let input = parse_file("D06_input.txt");
    let mut state = Part2::new(input);

    for _ in 0..256 {
        state.next_day();
    }

    tools::pretty_print_result_str(state.count().to_string());
}

pub fn parse_file(path: &str) -> Vec<i32> {
    let lines = tools::read_file_as_string(path);
    let fline = lines.iter().next().unwrap();
    let result: Vec<i32> = fline
        .split(",")
        .into_iter()
        .map(|numstr| numstr.parse().unwrap())
        .collect::<Vec<i32>>();
    return result;
}

struct Part2 {
    day: i32,
    state: Vec<i128>,
}

impl Part2 {
    pub fn new(init_state: Vec<i32>) -> Part2 {
        let mut state = vec![0; 9];
        for value in init_state {
            state[value as usize] += 1;
        }
        return Part2 {
            state: state.clone(),
            day: 0,
        };
    }

    pub fn next_day(&mut self) {
        const NY: usize = 8;
        const RESET: usize = 6;
        self.state.rotate_left(1);
        self.state[RESET] += self.state[NY];
        self.day += 1;
    }

    pub fn count(&self) -> i128 {
        return self.state.iter().sum::<i128>();
    }

    #[allow(dead_code)]
    pub fn print_state(&self) {
        println!("Day {}", self.day);
        for i in 0..8 {
            print!("{}", format!("{:>2}", i).green().bold());
        }
        println!("");
        for s in self.state.iter() {
            print!("{}", format!("{:>2}", s));
        }
        println!("");
    }
}

#[derive(Debug)]
struct Fish {
    timer: i32,
}

impl Fish {
    pub fn new(timer: i32) -> Fish {
        return Fish { timer };
    }

    pub fn reset(&mut self) {
        self.timer = 6;
    }

    pub fn next_day(&mut self) -> Option<Fish> {
        if &self.timer == &0 {
            self.reset();
            return Some(Fish::new(8));
        } else {
            self.timer -= 1;
            return None;
        }
    }
}

fn input_to_fishes(input: &Vec<i32>) -> Vec<Fish> {
    return input.iter().map(|num| Fish::new(num.clone())).collect();
}

#[derive(Debug)]
struct State {
    day: i32,
    fishes: Vec<Fish>,
}

impl State {
    pub fn new(fishes: Vec<Fish>) -> State {
        return State { fishes, day: 0 };
    }

    pub fn count(&self) -> i32 {
        return self.fishes.len() as i32;
    }

    pub fn next_day(&mut self) {
        let mut new_fishes: Vec<Fish> = vec![];
        self.day += 1;
        for fish in self.fishes.iter_mut() {
            let opfish = fish.next_day();

            if opfish.is_some() {
                new_fishes.push(opfish.unwrap());
            }
        }

        self.fishes.append(&mut new_fishes);
    }

    #[allow(dead_code)]
    pub fn print_state(&self) {
        println!("");
        if self.day == 0 {
            print!("Initial state: ");
        } else if self.day == 1 {
            print!("After {} day: ", &self.day)
        } else {
            print!("After {} days: ", &self.day)
        }

        for fish in self.fishes.iter() {
            print!("{}", format!("{:>2}", fish.timer));
        }
    }
}
