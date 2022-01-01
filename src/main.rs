mod days;
mod tools;

fn main() {
    tools::pretty_prints::pretty_print_banner();
    days::day1::solve_part1("D01_input.txt");
    days::day1::solve_part2("D01_input.txt");
}
