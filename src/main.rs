mod days;
mod tools;
use std::time::Instant;

fn main() {
    tools::pretty_prints::pretty_print_banner();

    let start = Instant::now();

    tools::time_utils::print_duration(days::day1::solve_part1);
    tools::time_utils::print_duration(days::day1::solve_part2);

    tools::time_utils::print_duration(days::day2::solve_part1);
    tools::time_utils::print_duration(days::day2::solve_part2);

    tools::time_utils::print_duration(days::day3::solve_part1);
    tools::time_utils::print_duration(days::day3::solve_part2);

    tools::time_utils::print_duration(days::day4::solve_part1);
    tools::time_utils::print_duration(days::day4::solve_part2);

    tools::time_utils::print_duration(days::day5::solve_part1);
    tools::time_utils::print_duration(days::day5::solve_part2);

    tools::time_utils::print_duration(days::day6::solve_part1);
    tools::time_utils::print_duration(days::day6::solve_part2);

    tools::time_utils::print_duration(days::day7::solve_part1);
    tools::time_utils::print_duration(days::day7::solve_part2);

    let duration = start.elapsed();

    println!("\nTotal time: {:?}", duration);
}
