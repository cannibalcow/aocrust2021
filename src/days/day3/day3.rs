use crate::tools;

pub fn solve_part1() {
    tools::pretty_print_day(3, 1);

    let lines: Vec<String> = tools::read_file_as_string("D03_input.txt");

    let mut gamma_rate = String::new();

    for n in 0..lines[0].len() {
        let common = most_common(&lines, n as usize);
        gamma_rate.push_str(&common);
    }

    let epsion_rate = invert_string(&gamma_rate);

    let result = string_binary_to_dec(&gamma_rate) * string_binary_to_dec(&epsion_rate);
    tools::pretty_print_result(result);
}

pub fn solve_part2() {
    tools::pretty_print_day(3, 2);
    let lines: Vec<String> = tools::read_file_as_string("D03_input.txt");
    let max_pos = lines[0].len();
    let oxygen = find_oxygen(&lines, 0, max_pos);
    let o2_scrub = find_c02_scrubber(&lines, 0, max_pos);

    tools::pretty_print_result(string_binary_to_dec(&o2_scrub) * string_binary_to_dec(&oxygen));
}

fn find_oxygen(lines: &Vec<String>, pos: usize, max_pos: usize) -> String {
    let common_char = most_common(&lines, pos).chars().next().unwrap();

    let result: Vec<String> = lines
        .iter()
        .cloned()
        .filter(|line| line.chars().nth(pos).unwrap() == common_char)
        .collect();

    if result.len() == 1 {
        return result[0].clone();
    }

    return find_oxygen(&result, pos + 1, max_pos);
}

fn find_c02_scrubber(lines: &Vec<String>, pos: usize, max_pos: usize) -> String {
    let least_common_char = least_common(&lines, pos).chars().next().unwrap();

    let result: Vec<String> = lines
        .iter()
        .cloned()
        .filter(|line| line.chars().nth(pos).unwrap() == least_common_char)
        .collect();

    if result.len() == 1 {
        return result[0].clone();
    }

    return find_c02_scrubber(&result, pos + 1, max_pos);
}

fn string_binary_to_dec(input: &str) -> i32 {
    let intval = isize::from_str_radix(input, 2).unwrap();
    return intval as i32;
}

fn invert_string(input: &str) -> String {
    let mut inverted = String::new();
    for n in 0..input.len() {
        let c = input.char_indices().nth(n).unwrap();
        if c.1 == '1' {
            inverted.push_str("0");
        } else {
            inverted.push_str("1");
        }
    }
    return inverted;
}

fn most_common(lines: &Vec<String>, pos: usize) -> String {
    let ones = lines
        .iter()
        .filter(|line| line.chars().nth(pos).unwrap() == '1')
        .count();

    let zeros = lines
        .iter()
        .filter(|line| line.chars().nth(pos).unwrap() == '0')
        .count();

    if zeros == ones {
        return "1".to_string();
    } else if zeros > ones {
        return "0".to_string();
    } else {
        return "1".to_string();
    }
}

fn least_common(lines: &Vec<String>, pos: usize) -> String {
    let ones = lines
        .iter()
        .filter(|line| line.chars().nth(pos).unwrap() == '1')
        .count();

    let zeros = lines
        .iter()
        .filter(|line| line.chars().nth(pos).unwrap() == '0')
        .count();

    if zeros == ones {
        return "0".to_string();
    } else if zeros < ones {
        return "0".to_string();
    } else {
        return "1".to_string();
    }
}
