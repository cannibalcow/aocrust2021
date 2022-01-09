use crate::tools;

pub fn solve_part1() {
    tools::pretty_print_day(7, 1);
    let values = tools::read_file_as_i32("D07_input.txt");

    let median = median(&mut values.clone());
    println!("Median: {}", median);

    let result: i32 = values
        .iter()
        .map(|v| {
            if v < &median {
                return median - v;
            }
            v - median
        })
        .sum();

    tools::pretty_print_result(result);
}

pub fn solve_part2() {
    tools::pretty_print_day(7, 2);
    let values = tools::read_file_as_i32("D07_input.txt");
    let sum: i32 = values.iter().sum();
    let medium: i32 = sum / values.len() as i32;

    let mut needed_fuel: i32 = 0;
    for value in &values {
        needed_fuel += (1..=(value - medium).abs()).fold(0, |a, b| a + b);
    }

    tools::pretty_print_result(needed_fuel);
}
fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
