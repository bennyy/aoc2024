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
    fn execute_part_1(&mut self, _input: &str) {
        self.part_1_result = Some("".to_string());
    }

    fn execute_part_2(&mut self, _input: &str) {
        self.part_2_result = Some("".to_string());
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
    use crate::filehelper::read_test_input;

    use super::*;

    #[test]
    fn test_day1_part_1() {
        let mut puzzle = Day1::new();
        let input = read_test_input(puzzle.day(), 1, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_day1_part_2() {
        let mut puzzle = Day1::new();
        let input = read_test_input(puzzle.day(), 2, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("".to_string()));
    }
}
