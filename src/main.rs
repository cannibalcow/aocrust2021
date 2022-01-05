mod days;
mod tools;

fn main() {
    tools::pretty_prints::pretty_print_banner();

    days::day1::solve_part1();
    days::day1::solve_part2();

    days::day2::solve_part1();
    days::day2::solve_part2();

    days::day3::solve_part1();
    days::day3::solve_part2();

    days::day4::solve_part1();
    days::day4::solve_part2();
}
