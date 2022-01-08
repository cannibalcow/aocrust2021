extern crate colored;
use colored::Colorize;

pub fn pretty_print_banner() {
    christmas_print("~~~~~~ Advent of code ~~~~~~");
}

pub fn christmas_print(text: &str) {
    text.chars().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            print!("{}", format!("{}", x).bold().red());
        } else {
            print!("{}", format!("{}", x).bold().white());
        }
    });
    println!("");
}

pub fn pretty_print_day(day: i32, part: i32) {
    christmas_print("============================");
    println!("Day {:?} Part {:?}", day, part);
}

pub fn pretty_print_result(result: i32) {
    if result == -1 {
        println!("Not done: {}", format!("{}", result).red());
        return;
    }
    println!("Result: {}", format!("{}", result).green());
}

pub fn pretty_print_result_str(result: String) {
    println!("Result: {}", format!("{}", result).green());
}
