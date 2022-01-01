mod tools;

fn main() {
    println!("Aoc 2021");
    solve_day1_part_1("d1_input.txt");
    solve_day1_part_2("d1_input.txt");
}

fn solve_day1_part_1(path: &str) {
    tools::pretty_print_day(1, 1);
    let result = tools::read_file_as_numbers(path)
        .windows(2)
        .map(|i| i[1] > i[0])
        .filter(|i| i == &true)
        .count() as i32;

    tools::pretty_print_result(result);
}

fn solve_day1_part_2(path: &str) {
    tools::pretty_print_day(1, 2);

    let result = tools::read_file_as_numbers(path)
        .windows(2)
        .map(|i| i[1] > i[0])
        .filter(|i| i == &true)
        .count() as i32;

    tools::pretty_print_result(result);
}
