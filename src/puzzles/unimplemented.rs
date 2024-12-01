use crate::adventofcode::Puzzle;

pub struct UnimplementedPuzzle {
    day: u32,
}

impl UnimplementedPuzzle {
    pub fn new(day: u32) -> Self {
        Self { day }
    }
}

impl Puzzle for UnimplementedPuzzle {
    fn day(&self) -> u32 {
        self.day
    }

    fn execute_part_1(&mut self, _input: &str) {
        println!("Part 1 is not implemented for day {}.", self.day);
    }

    fn execute_part_2(&mut self, _input: &str) {
        println!("Part 2 is not implemented for day {}.", self.day);
    }

    fn get_result_part_1(&self) -> Option<String> {
        None
    }

    fn get_result_part_2(&self) -> Option<String> {
        None
    }
}
