use crate::tools;

pub fn solve_part1() {
    tools::pretty_print_day(1, 1);
    let result = tools::read_file_as_numbers("D01_input.txt")
        .windows(2)
        .map(|i| i[1] > i[0])
        .filter(|i| i == &true)
        .count() as i32;

    tools::pretty_print_result(result);
}

pub fn solve_part2() {
    tools::pretty_print_day(1, 2);

    let result = tools::read_file_as_numbers("D01_input.txt")
        .windows(3)
        .map(|i| i.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|i| i[1] > i[0])
        .filter(|i| i == &true)
        .count() as i32;

    tools::pretty_print_result(result);
}
