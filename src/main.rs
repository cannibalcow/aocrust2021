mod days;
mod tools;
use std::time::Instant;

fn main() {
    tools::pretty_prints::pretty_print_banner();
    print_duration(days::day1::solve_part1);
    print_duration(days::day1::solve_part2);
    print_duration(days::day2::solve_part1);
    print_duration(days::day2::solve_part2);
    print_duration(days::day3::solve_part1);
    print_duration(days::day3::solve_part2);
    print_duration(days::day4::solve_part1);
    print_duration(days::day4::solve_part2);
}

fn print_duration(solve_func: fn()) {
    let start = Instant::now();
    solve_func();
    let duration = start.elapsed();
    println!("{:?}", duration);
}
