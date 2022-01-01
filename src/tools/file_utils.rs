use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_as_numbers(path: &str) -> Vec<i32> {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Can't open file: {:?} \n {:?}", path, error),
    };

    let bf = BufReader::new(file);

    let result = bf
        .lines()
        .map(|i| i.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return result;
}
