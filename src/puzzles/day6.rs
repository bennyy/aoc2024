use std::collections::HashSet;

use crate::adventofcode::Puzzle;

pub struct Day6 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day6 {
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

fn filter_direction(
    obstacles: &[(i32, i32)],
    start_pos: (i32, i32),
    direction: Direction,
) -> Option<&(i32, i32)> {
    match direction {
        Direction::N => obstacles
            .iter()
            .filter(|(x, y)| *x == start_pos.0 && start_pos.1 > *y)
            .min_by_key(|(_x, y)| (*y - start_pos.1).abs()),
        Direction::S => obstacles
            .iter()
            .filter(|(x, y)| *x == start_pos.0 && start_pos.1 < *y)
            .min_by_key(|(_x, y)| (*y - start_pos.1).abs()),
        Direction::E => obstacles
            .iter()
            .filter(|(x, y)| *x > start_pos.0 && start_pos.1 == *y)
            .min_by_key(|(x, _y)| (*x - start_pos.0).abs()),
        Direction::W => obstacles
            .iter()
            .filter(|(x, y)| *x < start_pos.0 && start_pos.1 == *y)
            .min_by_key(|(x, _y)| (*x - start_pos.0).abs()),
    }
}

fn run_simulation(
    obstacles: &[(i32, i32)],
    start_pos: (i32, i32),
    width: i32,
    height: i32,
) -> Option<HashSet<(i32, i32)>> {
    let mut places: HashSet<(i32, i32)> = HashSet::new();
    let path = vec![Direction::N, Direction::E, Direction::S, Direction::W];
    let mut directions = path.into_iter().cycle();
    let mut current_pos = start_pos;

    let mut last_size = 0;
    let mut loop_count = 0;
    loop {
        if let Some(direction) = directions.next() {
            if let Some(obstacle) = filter_direction(obstacles, current_pos, direction) {
                let x_steps =
                    direction.coordinates().0.abs() * ((obstacle.0 - current_pos.0).abs() - 1);
                let y_steps =
                    direction.coordinates().1.abs() * ((obstacle.1 - current_pos.1).abs() - 1);
                let tmp_steps = x_steps + y_steps;

                for i in 0..tmp_steps {
                    let x = current_pos.0 + direction.coordinates().0 * i;
                    let y = current_pos.1 + direction.coordinates().1 * i;
                    places.insert((x, y));
                }

                current_pos.0 += tmp_steps * direction.coordinates().0;
                current_pos.1 += tmp_steps * direction.coordinates().1;

                if places.len() != last_size {
                    loop_count = 0;
                } else {
                    loop_count += 1;

                    if loop_count == 4 {
                        // If we loop 4 times, we're probably stuck in a loop
                        return None;
                    }
                }

                last_size = places.len();
            } else {
                let tmp_steps = match direction {
                    Direction::N => current_pos.1.abs(),
                    Direction::S => (current_pos.1 - (height - 1)).abs(),
                    Direction::W => current_pos.0.abs(),
                    Direction::E => (current_pos.0 - (width - 1)).abs(),
                };

                for i in 0..=tmp_steps {
                    let x = current_pos.0 + direction.coordinates().0 * i;
                    let y = current_pos.1 + direction.coordinates().1 * i;
                    places.insert((x, y));
                }

                current_pos.0 += tmp_steps * direction.coordinates().0;
                current_pos.1 += tmp_steps * direction.coordinates().1;
                // End of the map!
                return Some(places);
            }
        }
    }
}

impl Puzzle for Day6 {
    fn execute_part_1(&mut self, input: &str) {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let map: Vec<char> = input.lines().collect::<String>().chars().collect();
        let obstacles: Vec<(i32, i32)> = map
            .iter()
            .enumerate()
            .filter(|(_i, c)| **c == '#')
            .map(|(i, _c)| (i as i32 % width, i as i32 / width))
            .collect();
        let start_pos_index = map.iter().enumerate().find(|(_i, c)| **c == '^').unwrap().0 as i32;
        let start_pos: (i32, i32) = (start_pos_index % width, start_pos_index / width);

        if let Some(places) = run_simulation(&obstacles, start_pos, width, height) {
            self.part_1_result = Some(places.len().to_string());
        }
    }

    fn execute_part_2(&mut self, input: &str) {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let map: Vec<char> = input.lines().collect::<String>().chars().collect();
        let obstacles: Vec<(i32, i32)> = map
            .iter()
            .enumerate()
            .filter(|(_i, c)| **c == '#')
            .map(|(i, _c)| (i as i32 % width, i as i32 / width))
            .collect();
        let start_pos_index = map.iter().enumerate().find(|(_i, c)| **c == '^').unwrap().0 as i32;
        let start_pos: (i32, i32) = (start_pos_index % width, start_pos_index / width);

        let mut result = 0;
        if let Some(mut places) = run_simulation(&obstacles, start_pos, width, height) {
            // Remove starting position
            places.retain(|&k| k != start_pos);

            for place in places.iter() {
                let mut new_obstacles = obstacles.clone();
                new_obstacles.push(*place);

                let places = run_simulation(&new_obstacles, start_pos, width, height);
                if places.is_none() {
                    // None == infinite loop
                    result += 1;
                }
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
        6
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day6_part_1() {
        let mut puzzle = Day6::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("41".to_string()));
    }

    #[test]
    fn test_day6_part_1_real() {
        let mut puzzle = Day6::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("5329".to_string()));
    }

    #[test]
    fn test_day6_part_2() {
        let mut puzzle = Day6::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("6".to_string()));
    }

    #[test]
    fn test_day6_part_2_real() {
        let mut puzzle = Day6::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("2162".to_string()));
    }
}
