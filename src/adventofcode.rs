use std::collections::HashMap;

use crate::puzzles::unimplemented::UnimplementedPuzzle;

pub trait Puzzle {
    fn day(&self) -> u32;
    fn execute_part_1(&mut self, input: &str);
    fn execute_part_2(&mut self, input: &str);
    fn get_result_part_1(&self) -> Option<String>;
    fn get_result_part_2(&self) -> Option<String>;
}

pub struct AdventOfCode {
    pub puzzles: HashMap<u32, Box<dyn Puzzle>>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        Self {
            puzzles: HashMap::new(),
        }
    }

    pub fn add_puzzle(&mut self, day: u32, puzzle: Box<dyn Puzzle>) {
        self.puzzles.insert(day, puzzle);
    }

    pub fn get_puzzle(&mut self, day: u32) -> &mut dyn Puzzle {
        self.puzzles
            .entry(day)
            .or_insert_with(|| Box::new(UnimplementedPuzzle::new(day)))
            .as_mut()
    }
}
