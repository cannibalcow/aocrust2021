use crate::tools;
use std::fmt;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn solve_part1() {
    tools::pretty_print_day(2, 1);
    let mut position = Position::new(0, 0, 0);
    let commands = file_to_command("D02_input.txt");

    for command in commands {
        match command.direction {
            Direction::Forward => position.forward(command.value),
            Direction::Up => position.up(command.value),
            Direction::Down => position.down(command.value),
        }
    }

    tools::pretty_print_result(position.horizontal * position.depth);
}

pub fn solve_part2() {
    tools::pretty_print_day(2, 2);
    let mut position = Position::new(0, 0, 0);
    let commands = file_to_command("D02_input.txt");

    for command in commands {
        match command.direction {
            Direction::Forward => position.forward_with_aim(command.value),
            Direction::Up => position.aim_up(command.value),
            Direction::Down => position.aim_down(command.value),
        }
    }

    tools::pretty_print_result(position.horizontal * position.depth);
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new(horizontal: i32, depth: i32, aim: i32) -> Position {
        return Position {
            horizontal: horizontal,
            depth: depth,
            aim: aim,
        };
    }

    fn forward(&mut self, value: i32) {
        self.horizontal += value;
    }
    fn forward_with_aim(&mut self, value: i32) {
        self.horizontal += value;
        self.depth += self.aim * value;
    }

    fn aim_down(&mut self, value: i32) {
        self.aim += value;
    }

    fn aim_up(&mut self, value: i32) {
        self.aim -= value;
    }

    fn down(&mut self, value: i32) {
        self.depth += value;
    }
    fn up(&mut self, value: i32) {
        self.depth -= value;
    }
}

struct Command {
    direction: Direction,
    value: i32,
}

impl Command {
    fn new(direction: Direction, value: i32) -> Command {
        return Command {
            direction: direction,
            value: value,
        };
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f
            .debug_struct("Command")
            .field("direction", &self.direction.to_string())
            .field("value", &self.value)
            .finish();
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f
            .debug_struct("Position")
            .field("horizontal", &self.horizontal)
            .field("depth", &self.depth)
            .field("aim", &self.aim)
            .finish();
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Forward => write!(f, "Forward"),
            Direction::Down => write!(f, "Down"),
            Direction::Up => write!(f, "Up"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = Direction;
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(Direction::Down),
        }
    }
}

fn file_to_command(path: &str) -> Vec<Command> {
    let file = tools::get_file(path);

    let bf = BufReader::new(file);

    let result = bf
        .lines()
        .map(|it| {
            let line = String::from(it.unwrap());
            let mut split = line.split_whitespace();
            let first_arg = split.next().expect("Direction");
            let cm = Direction::from_str(first_arg);
            let value = split.next().unwrap().parse::<i32>().unwrap();
            return Command::new(cm.unwrap(), value);
        })
        .collect::<Vec<Command>>();

    return result;
}
