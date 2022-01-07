extern crate bresenham;

use bresenham::Bresenham;
use colored::Colorize;

use crate::tools::{self, pretty_print_day, pretty_print_result};

pub fn solve_part1() {
    pretty_print_day(5, 1);

    let lines = parse_file("D05_input.txt");
    let grid_size: (u32, u32) = calculate_grid_size(&lines);

    let mut grid = Grid::new(grid_size.1 as usize, grid_size.0 as usize);
    grid.fill_lines(lines, true);
    pretty_print_result(grid.count_where_more_than_two());
}

pub fn solve_part2() {
    pretty_print_day(5, 2);
    let lines = parse_file("D05_input.txt");
    let grid_size: (u32, u32) = calculate_grid_size(&lines);

    let mut grid = Grid::new(grid_size.1 as usize, grid_size.0 as usize);
    grid.fill_lines(lines, false);
    pretty_print_result(grid.count_where_more_than_two());
}

fn calculate_grid_size(lines: &Vec<Line>) -> (u32, u32) {
    let max = lines.iter().fold((0, 0), |acc, item| {
        // width
        let width_values = vec![acc.0, item.p1.0, item.p2.0];
        let max_width = width_values.iter().max().unwrap();

        let height_value = vec![acc.1, item.p1.1, item.p2.1];
        let max_height = height_value.iter().max().unwrap();

        return (max_width.clone(), max_height.clone());
    });

    return max;
}

fn parse_file(path: &str) -> Vec<Line> {
    let lines = tools::read_file_as_string(path)
        .iter()
        .map(|f| parse_row(f))
        .collect::<Vec<Line>>();

    return lines;
}

fn parse_row(row: &str) -> Line {
    let mut two_parts = row.split(" -> ").enumerate();
    let f1 = two_parts.next().unwrap().1;
    let f2 = two_parts.next().unwrap().1;
    let p1 = to_tupple(&f1);
    let p2 = to_tupple(&f2);

    return Line::new(p1, p2);
}

// "2,2" -> (2,2)
fn to_tupple(value: &str) -> (u32, u32) {
    let mut p = value.split(",").enumerate();

    let p1 = &p.next().unwrap().1.parse::<u32>().unwrap();
    let p2 = &p.next().unwrap().1.parse::<u32>().unwrap();

    return (*p1, *p2);
}
#[derive(Debug)]
struct Line {
    p1: (u32, u32),
    p2: (u32, u32),
}

impl Line {
    fn new(p1: (u32, u32), p2: (u32, u32)) -> Line {
        return Line { p1, p2 };
    }

    fn to_bp1(&self) -> (isize, isize) {
        return (self.p1.0 as isize, self.p1.1 as isize);
    }

    fn to_bp2(&self) -> (isize, isize) {
        return (self.p2.0 as isize, self.p2.1 as isize);
    }

    fn is_horizontal(&self) -> bool {
        let is_horizontal = self.x1() == self.x2() || self.y1() == self.y2();
        return is_horizontal;
    }

    fn x1(&self) -> u32 {
        return self.p1.0;
    }
    fn x2(&self) -> u32 {
        return self.p2.0;
    }

    fn y1(&self) -> u32 {
        return self.p1.1;
    }

    fn y2(&self) -> u32 {
        return self.p2.1;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Grid {
    vec: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let vec = vec![vec![0; width + 1]; height + 1];
        Self {
            vec,
            width: width + 1,
            height: height + 1,
        }
    }

    pub fn count_where_more_than_two(&self) -> i32 {
        let mut c = 0;

        for (row, value) in self.vec.iter().enumerate() {
            for (col, _value) in value.iter().enumerate() {
                if self.vec[row][col] > 1 {
                    c = c + 1;
                }
            }
        }
        return c as i32;
    }
    
    #[allow(dead_code)]
    pub fn prind_grid(&self) {
        println!("{}x{}", self.height, self.width);
        print!("{}", format!("{:>2}", " ").yellow().bold());
        for n in 0..self.width {
            print!("{}", format!("{:>2}", n).yellow().bold());
        }
        println!("");
        for (row, value) in self.vec.iter().enumerate() {
            print!("{}", format!("{:>2}", row).yellow().bold());
            for (_col, val) in value.iter().enumerate() {
                match val {
                    0 => print!("{}", format!("{:>2}", val).black()),
                    1 => print!("{}", format!("{:>2}", val).green()),
                    2 => print!("{}", format!("{:>2}", val).red()),
                    3 => print!("{}", format!("{:>2}", val).yellow()),
                    _ => print!("{}", format!("{:>2}", val)),
                }
            }
            println!("");
        }
    }

    pub fn fill_lines(&mut self, lines: Vec<Line>, skip_diagonal: bool) {
        for line in lines {
            if skip_diagonal && !line.is_horizontal() {
                continue;
            } else {
                for (x, y) in Bresenham::new(line.to_bp1(), line.to_bp2()) {
                    let  val = self.vec[x as usize][y as usize];
                    self.vec[x as usize][y as usize] = val +1;
                }
                let new_val = self.vec[line.p2.0 as usize][line.p2.1 as usize];
                self.vec[line.p2.0 as usize][line.p2.1 as usize] = new_val + 1;
            }
        }
    }

    
}
