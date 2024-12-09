use crate::adventofcode::Puzzle;

pub struct Day<X> {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day<X> {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

impl Puzzle for Day<X> {
    fn execute_part_1(&mut self, input: &str) {
        self.part_1_result = Some("0".len().to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        self.part_2_result = Some("0".len().to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        <X>
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day<X>_part_1() {
        let mut puzzle = Day<X>::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("14".to_string()));
    }

    #[test]
    fn test_day<X>_part_1_real() {
        let mut puzzle = Day<X>::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("220".to_string()));
    }

    #[test]
    fn test_day<X>_part_2() {
        let mut puzzle = Day<X>::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }

    #[test]
    fn test_day<X>_part_2_real() {
        let mut puzzle = Day<X>::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }
}
