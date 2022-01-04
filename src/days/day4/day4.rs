extern crate colored;

use crate::tools;
use colored::Colorize;

pub fn solve_part1() {
    let input_path = "D04_input_test.txt";
    let numbers = parse_input_to_numbers(&input_path);
    let mut boards = parse_input_to_boards(&input_path);

    println!("{:?}", numbers);

    println!("== Boards");

    for num in &numbers {
        println!("\n Marking {}", num);
        for board in &mut boards {
            board.mark(num.clone());
            board.print_board();
            println!("Bingo: {}", board.has_bingo());

            if board.has_bingo() {
                let sum = board.sum_of_all_unmarked();
                println!("Sum: {}", sum);
                println!("Num: {}", num);
                println!("Reuslt: {}", sum * num);
                tools::pretty_print_result((num * sum) as i32);
                return;
            }
        }
    }

    // for board in boards {
    // board.print_board();
    // }
}

pub fn solve_part2() {}

fn parse_input_to_boards(path: &str) -> Vec<BingoBoard> {
    let lines = tools::read_file_as_string(path);
    let mut rows: Vec<u32> = vec![];
    let mut boards: Vec<BingoBoard> = vec![];

    for _i in 0..lines.len() + 1 {
        if _i == 0 {
            continue;
        }

        if _i >= lines.len() {
            boards.push(BingoBoard::new(rows.clone(), vec![]));
            break;
        }

        if lines[_i] == String::from("") {
            if _i == 1 {
                continue;
            }
            boards.push(BingoBoard::new(rows.clone(), vec![]));
            rows = vec![];
            continue;
        } else {
            let line = &lines[_i];
            let trimed_line = line.replace("  ", " ");

            let mut row: Vec<u32> = trimed_line
                .trim()
                .split(' ')
                .map(|f| f.parse::<u32>().unwrap())
                .collect();

            rows.append(&mut row);
        }
    }

    return boards;
}

fn parse_input_to_numbers(path: &str) -> Vec<u32> {
    let lines = tools::read_file_as_string(path);
    println!("Lines {}", lines.len());
    let numbers = lines[0]
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return numbers;
}

struct BingoBoard {
    numbers: Vec<u32>,
    marked_numbers: Vec<u32>,
}

impl BingoBoard {
    fn new(numbers: Vec<u32>, marked_numbers: Vec<u32>) -> BingoBoard {
        return BingoBoard {
            numbers: numbers,
            marked_numbers: marked_numbers,
        };
    }

    fn sum_of_all_unmarked(&self) -> u32 {
        return self
            .numbers
            .iter()
            .filter(|num| !self.marked_numbers.contains(num))
            .sum::<u32>();
    }

    fn has_bingo(&self) -> bool {
        let rows: Vec<&[u32]> = self.numbers.chunks(5).collect();

        for row in rows {
            let mut count = 0;
            for i in row {
                if self.marked_numbers.contains(i) {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        return false;
    }

    fn print_board(&self) {
        println!("");
        for (i, value) in self.numbers.iter().enumerate() {
            let is_marked = self.marked_numbers.contains(value);

            if i % 5 != 0 {
                if is_marked {
                    print!("{}", format!("{:>4}", value).bold().bright_yellow());
                } else {
                    print!("{}", format!("{:>4}", value));
                }
            } else {
                println!();
                if is_marked {
                    print!("{}", format!("{:>4}", value).bold().bright_yellow());
                } else {
                    print!("{}", format!("{:>4}", value));
                }
            }
        }
        println!("");
    }

    fn mark(&mut self, num: u32) {
        let index = self.numbers.iter().position(|&r| r == num);

        if index.is_none() {
            return;
        }

        self.marked_numbers.push(num);
    }
}
