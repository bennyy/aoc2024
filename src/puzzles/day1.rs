use crate::adventofcode::Puzzle;

pub struct Day1 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day1 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

impl Puzzle for Day1 {
    fn execute_part_1(&mut self, input: &str) {
        // Split the input lines into two vectors of integers
        let (mut left_values, mut right_values): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|item| {
                let mut parts = item.split_whitespace();
                (
                    parts.next().unwrap().parse::<i32>().unwrap(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        // Sort the left and right values independently
        left_values.sort();
        right_values.sort();

        // Calculate the total distance between each left value and its corresponding right value
        let total_distance = left_values
            .iter()
            .zip(right_values.iter())
            .map(|(left, right)| ((left - right).abs()))
            .sum::<i32>();

        self.part_1_result = Some(total_distance.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        // Split the input lines into two vectors of integers
        let (left_values, right_values): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|item| {
                let mut parts = item.split_whitespace();
                (
                    parts.next().unwrap().parse::<i32>().unwrap(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        // Calculate the similarity score by summing up the products
        // of each left value and its occurrences in the right values
        let similarity_score = left_values
            .iter()
            .map(|&value| right_values.iter().filter(|&&x| x == value).count() as i32 * value)
            .sum::<i32>();

        self.part_2_result = Some(similarity_score.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day1_part_1() {
        let mut puzzle = Day1::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("11".to_string()));
    }

    #[test]
    fn test_day2_part_1_real() {
        let mut puzzle = Day1::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("2113135".to_string()));
    }

    #[test]
    fn test_day1_part_2() {
        let mut puzzle = Day1::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("31".to_string()));
    }

    #[test]
    fn test_day1_part_2_real() {
        let mut puzzle = Day1::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("19097157".to_string()));
    }
}
