use std::collections::{HashSet, VecDeque};

use crate::adventofcode::Puzzle;

pub struct Day12 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn coordinates(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
        }
    }
}

impl Puzzle for Day12 {
    fn execute_part_1(&mut self, input: &str) {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let map: Vec<char> = input
            .lines()
            .flat_map(|line| line.chars())
            .collect();

        let mut map_copy = map.clone();
        let mut result = 0;

        let mut ignore_indexes: HashSet<usize> = HashSet::new();

        for (i, c) in map_copy.iter_mut().enumerate() {
            if ignore_indexes.contains(&i) {
                continue;
            }
            let target = *c;
            let mut perimeter = 0;
            let mut queue = VecDeque::new();
            let mut visited: HashSet<usize> = HashSet::new();
            queue.push_back(i);
            //visited.insert(i);
            while let Some(index) = queue.pop_front() {
                if visited.contains(&index) {
                    continue;
                }
                visited.insert(index);

                let x = index as i32 % width;
                let y = index as i32 / width;

                for &dir in &[Direction::N, Direction::S, Direction::W, Direction::E] {
                    let new_x = x + dir.coordinates().0;
                    let new_y = y + dir.coordinates().1;

                    if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                        let n_index = new_x as usize + width as usize * new_y as usize;
                        if *map.get(n_index).unwrap() != target && !visited.contains(&n_index) {
                            perimeter += 1;
                        } else if !visited.contains(&n_index) {
                            queue.push_back(n_index);
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            ignore_indexes.extend(visited.clone());
            result += perimeter * visited.len();
        }

        self.part_1_result = Some(result.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        self.part_2_result = Some("0".to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        12
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day12_part_1_1() {
        let mut puzzle = Day12::new();
        let input = read_test_input(puzzle.day(), None, Some(1));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("140".to_string()));
    }

    #[test]
    fn test_day12_part_1_2() {
        let mut puzzle = Day12::new();
        let input = read_test_input(puzzle.day(), None, Some(2));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("772".to_string()));
    }
    #[test]

    fn test_day12_part_1_3() {
        let mut puzzle = Day12::new();
        let input = read_test_input(puzzle.day(), None, Some(3));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("1930".to_string()));
    }

    #[test]
    fn test_day12_part_1_real() {
        let mut puzzle = Day12::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("1464678".to_string()));
    }

    #[test]
    fn test_day12_part_2() {
        let mut puzzle = Day12::new();
        let input = read_test_input(puzzle.day(), None, Some(1));

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }

    #[test]
    fn test_day12_part_2_real() {
        let mut puzzle = Day12::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }
}
