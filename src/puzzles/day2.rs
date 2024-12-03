use crate::adventofcode::Puzzle;

pub struct Day2 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day2 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

fn check_report(row: &[i32]) -> bool {
    let increases = row
        .windows(2)
        .filter(|window| {
            if let [a, b] = *window {
                b > a && (b - a).abs() <= 3
            } else {
                false
            }
        })
        .count();
    let decreases = row
        .windows(2)
        .filter(|window| {
            if let [a, b] = *window {
                b < a && (b - a).abs() <= 3
            } else {
                false
            }
        })
        .count();

    let expected = row.len() - 1;

    increases == expected || decreases == expected
}

impl Puzzle for Day2 {
    fn execute_part_1(&mut self, input: &str) {
        let numbers: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let safe_reports = numbers.iter().filter(|row| check_report(row)).count();

        self.part_1_result = Some(safe_reports.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let numbers: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut safe_reports = 0;
        numbers.iter().for_each(|row| {
            if check_report(row) {
                safe_reports += 1;
            } else {
                let combinations: Vec<Vec<i32>> = row
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(j, _)| i != j)
                            .map(|(_, &x)| x)
                            .collect()
                    })
                    .collect();

                if combinations.iter().any(|row| check_report(row)) {
                    safe_reports += 1;
                }
            }
        });

        self.part_2_result = Some(safe_reports.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day2_part_1() {
        let mut puzzle = Day2::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("2".to_string()));
    }

    #[test]
    fn test_day2_part_1_real() {
        let mut puzzle = Day2::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("369".to_string()));
    }

    #[test]
    fn test_day2_part_2() {
        let mut puzzle = Day2::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("4".to_string()));
    }

    #[test]
    fn test_day2_part_2_real() {
        let mut puzzle = Day2::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("428".to_string()));
    }
}
