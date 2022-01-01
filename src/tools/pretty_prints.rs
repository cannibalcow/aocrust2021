extern crate colored;
use colored::Colorize;

pub fn pretty_print_day(day: i32, part: i32) {
    println!("=======================================");
    println!("Day {:?} Part {:?}", day, part);
}

pub fn pretty_print_result(result: i32) {
    println!("Result: {}", format!("{}", result).green());
}
