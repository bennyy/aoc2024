use std::collections::HashSet;

use crate::adventofcode::Puzzle;

pub struct Day10 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day10 {
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

fn parse_input(input: &str) -> (Vec<u32>, Vec<(i32, i32)>) {
    let width = input.lines().next().unwrap().len() as i32;
    let map: Vec<u32> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(u32::MAX)))
        .collect();
    let starting_pos: Vec<(i32, i32)> = map
        .iter()
        .enumerate()
        .filter(|(_, &number)| number == 0)
        .map(|(index, _)| {
            let x = index as i32 % width;
            let y = index as i32 / width;
            (x, y)
        })
        .collect();

    (map, starting_pos)
}

fn find_all_paths(
    map: &[u32],
    width: i32,
    height: i32,
    start_pos: (i32, i32),
) -> HashSet<Vec<(i32, i32)>> {
    let mut all_paths = HashSet::new();
    let mut stack = vec![(vec![start_pos], start_pos)]; // Current path, and current position)

    while let Some((path, (x, y))) = stack.pop() {
        let this_index = x as usize + width as usize * y as usize;

        if map[this_index] == 9 {
            // Found a valid path!
            all_paths.insert(path.clone());
            continue;
        }

        for &dir in &[Direction::N, Direction::S, Direction::W, Direction::E] {
            let new_x = x + dir.coordinates().0;
            let new_y = y + dir.coordinates().1;

            if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                let new_pos = (new_x, new_y);
                let index = new_x as usize + width as usize * new_y as usize;
                let next_valid_step = map[this_index] + 1;

                if next_valid_step != map[index] {
                    continue;
                }

                if map[index] == next_valid_step && !path.contains(&new_pos) {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    stack.push((new_path, new_pos));
                }
            }
        }
    }

    all_paths
}

impl Puzzle for Day10 {
    fn execute_part_1(&mut self, input: &str) {
        let mut result = 0;
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let (map, starting_pos) = parse_input(input);

        for start in &starting_pos {
            let all_paths = find_all_paths(&map, width, height, *start);

            let unique_paths: HashSet<(i32, i32)> =
                all_paths.iter().map(|path| *path.last().unwrap()).collect();

            result += unique_paths.len();
        }

        self.part_1_result = Some(result.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let mut result = 0;
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let (map, starting_pos) = parse_input(input);

        for start in &starting_pos {
            let all_paths = find_all_paths(&map, width, height, *start);
            result += all_paths.len();
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
        10
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day10_part_1_1() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(1));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("1".to_string()));
    }

    #[test]
    fn test_day10_part_1_2() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(2));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("2".to_string()));
    }

    #[test]
    fn test_day10_part_1_3() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(3));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("4".to_string()));
    }

    #[test]
    fn test_day10_part_1_4() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(4));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("3".to_string()));
    }

    #[test]
    fn test_day10_part_1_5() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(5));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("36".to_string()));
    }

    #[test]
    fn test_day10_part_1_real() {
        let mut puzzle = Day10::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("550".to_string()));
    }

    #[test]
    fn test_day10_part_2() {
        let mut puzzle = Day10::new();
        let input = read_test_input(puzzle.day(), None, Some(5));

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("81".to_string()));
    }

    #[test]
    fn test_day10_part_2_real() {
        let mut puzzle = Day10::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("1255".to_string()));
    }
}
