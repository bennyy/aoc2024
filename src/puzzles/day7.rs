use std::collections::HashSet;

use crate::adventofcode::Puzzle;

pub struct Day7 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day7 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Mul,
    Con,
}

fn calibration_equation(
    current_value: i64,
    remaining: &[i64],
    operator: Operator,
    results: &mut HashSet<i64>,
    goal: i64,
    allow_concatenation: bool,
) {
    if let Some((&next_value, rest)) = remaining.split_first() {
        let new_value = match operator {
            Operator::Add => current_value + next_value,
            Operator::Mul => current_value * next_value,
            Operator::Con => {
                format!("{}{}", current_value, next_value)
                    .parse::<i64>()
                    .unwrap_or(0)
            }
        };

        if new_value <= goal {
            for &next_operator in &[Operator::Add, Operator::Mul] {
                calibration_equation(new_value, rest, next_operator, results, goal, allow_concatenation);
            }
            if allow_concatenation {
                calibration_equation(new_value, rest, Operator::Con, results, goal, allow_concatenation);
            }
        }
    } else if current_value == goal {
        results.insert(current_value);
    }
}

impl Puzzle for Day7 {
    fn execute_part_1(&mut self, input: &str) {
        let mut result = 0;
        for line in input.lines() {
            let mut splitted_line = line.split(":");
            let answer = splitted_line.next().unwrap().parse::<i64>().unwrap();
            let values = splitted_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut results: HashSet<i64> = HashSet::new();
            let (current_value, remaining) = values.split_first().unwrap();

            calibration_equation(
                *current_value,
                remaining,
                Operator::Add,
                &mut results,
                answer,
                false,
            );

            calibration_equation(
                *current_value,
                remaining,
                Operator::Mul,
                &mut results,
                answer,
                false,
            );

            if results.contains(&answer) {
                result += answer;
            }
        }
        self.part_1_result = Some(result.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let mut result = 0;
        for line in input.lines() {
            let mut splitted_line = line.split(":");
            let answer = splitted_line.next().unwrap().parse::<i64>().unwrap();
            let values = splitted_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut results: HashSet<i64> = HashSet::new();
            let (current_value, remaining) = values.split_first().unwrap();

            calibration_equation(
                *current_value,
                remaining,
                Operator::Add,
                &mut results,
                answer,
                true,
            );

            calibration_equation(
                *current_value,
                remaining,
                Operator::Mul,
                &mut results,
                answer,
                true,
            );

            calibration_equation(
                *current_value,
                remaining,
                Operator::Con,
                &mut results,
                answer,
                true,
            );

            if results.contains(&answer) {
                result += answer;
            }
        }
        self.part_2_result = Some(result.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        7
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day7_part_1() {
        let mut puzzle = Day7::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("3749".to_string()));
    }

    #[test]
    fn test_day7_part_1_real() {
        let mut puzzle = Day7::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("3119088655389".to_string()));
    }

    #[test]
    fn test_day7_part_2() {
        let mut puzzle = Day7::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("11387".to_string()));
    }

    #[test]
    fn test_day7_part_2_real() {
        let mut puzzle = Day7::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("264184041398847".to_string()));
    }
}
