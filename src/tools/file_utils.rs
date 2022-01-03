pub mod file_utils {

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

    pub fn read_file_as_string(path: &str) -> Vec<String> {
        let file = get_file(path);

        let bf = BufReader::new(file);

        let result = bf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        return result;
    }

    pub fn get_file(path: &str) -> File {
        let file = File::open(path);
        let file = match file {
            Ok(file) => file,
            Err(error) => panic!("Can't open file: {:?} \n {:?}", path, error),
        };

        return file;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn read_file_as_numbers_test() {
            let result = read_file_as_numbers("test_numbers.txt");

            assert_eq!(result, [1, 2, 3, 4, 5]);
        }
    }
}
