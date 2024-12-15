use std::collections::HashMap;

use crate::adventofcode::Puzzle;

pub struct Day11 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day11 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

fn calc_stones(input: &str, steps: usize) -> u64 {
    let mut starting_stones = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .for_each(|num| *starting_stones.entry(num).or_insert(0) += 1);

    let mut starting_stones_clone = starting_stones.clone();
    for _ in 0..steps {
        let mut tmp: HashMap<u64, u64> = HashMap::new();

        for (value, occurences) in starting_stones_clone {
            if value == 0 {
                //If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
                *tmp.entry(1).or_insert(0) += occurences;
            } else if value.to_string().len() % 2 == 0 {
                // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone, and the right half of the digits are
                // engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)

                // Convert the number to a string
                let num_str = value.to_string();
                let len = num_str.len();
                let mid = len / 2;

                // Split the string into two halves
                let left_part = &num_str[..mid]; // First half
                let right_part = &num_str[mid..]; // Second half

                // Convert back to numbers (default to 0 if empty)
                let left = left_part.parse::<u64>().unwrap_or(0);
                let right = right_part.parse::<u64>().unwrap_or(0);

                *tmp.entry(left).or_insert(0) += occurences;
                *tmp.entry(right).or_insert(0) += occurences;
            } else {
                // If none of the other rules apply, the stone is replaced by a new stone;
                // the old stone's number multiplied by 2024 is engraved on the new stone.
                *tmp.entry(value * 2024).or_insert(0) += occurences;
            }
        }

        starting_stones_clone = tmp.clone();
    }

    starting_stones_clone.values().sum()
}

impl Puzzle for Day11 {
    fn execute_part_1(&mut self, input: &str) {
        self.part_1_result = Some(calc_stones(input, 25).to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        self.part_2_result = Some(calc_stones(input, 75).to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        11
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day11_part_1() {
        let mut puzzle = Day11::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("55312".to_string()));
    }

    #[test]
    fn test_day11_part_1_real() {
        let mut puzzle = Day11::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("199982".to_string()));
    }

    #[test]
    fn test_day11_part_2() {
        let mut puzzle = Day11::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("65601038650482".to_string()));
    }

    #[test]
    fn test_day11_part_2_real() {
        let mut puzzle = Day11::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("237149922829154".to_string()));
    }
}
