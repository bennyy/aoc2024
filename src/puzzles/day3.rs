use regex::Regex;

use crate::adventofcode::Puzzle;

pub struct Day3 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day3 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

impl Puzzle for Day3 {
    fn execute_part_1(&mut self, input: &str) {
        let pattern = r"mul\((\d+),(\d+)\)";
        let re = Regex::new(pattern).unwrap();
        let sum: i32 = re
            .captures_iter(input)
            .map(|x| x[1].parse::<i32>().unwrap() * x[2].parse::<i32>().unwrap())
            .sum();

        self.part_1_result = Some(sum.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let pattern = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
        let re = Regex::new(pattern).unwrap();

        let mut skip = false;
        let sum: i32 = re
            .captures_iter(input)
            .map(|x| {
                let func_name = x.get(0).unwrap().as_str();

                if func_name.contains("don't") {
                    skip = true;
                } else if func_name.contains("do") {
                    skip = false;
                } else if func_name.contains("mul") && !skip {
                    return x[1].parse::<i32>().unwrap() * x[2].parse::<i32>().unwrap();
                }
                0
            })
            .sum();

        self.part_2_result = Some(sum.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        3
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day3_part_1() {
        let mut puzzle = Day3::new();
        let input = read_test_input(puzzle.day(), Some(1), None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("161".to_string()));
    }

    #[test]
    fn test_day3_part_1_real() {
        let mut puzzle = Day3::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("163931492".to_string()));
    }

    #[test]
    fn test_day3_part_2() {
        let mut puzzle = Day3::new();
        let input = read_test_input(puzzle.day(), Some(2), None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("48".to_string()));
    }

    #[test]
    fn test_day3_part_2_real() {
        let mut puzzle = Day3::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("76911921".to_string()));
    }
}
