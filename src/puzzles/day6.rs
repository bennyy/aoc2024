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
    obstacles: &Vec<(i32, i32)>,
    start_pos: (i32, i32),
    direction: Direction,
) -> Option<&(i32, i32)> {
    match direction {
        Direction::N => obstacles
            .iter()
            .filter(|(x, y)| *x == start_pos.0 && start_pos.1 > *y)
            .min_by_key(|(x, y)| (*y - start_pos.1).abs()),
        Direction::S => obstacles
            .iter()
            .filter(|(x, y)| *x == start_pos.0 && start_pos.1 < *y)
            .min_by_key(|(x, y)| (*y - start_pos.1).abs()),
        Direction::E => obstacles
            .iter()
            .filter(|(x, y)| *x > start_pos.0 && start_pos.1 == *y)
            .min_by_key(|(x, y)| (*x - start_pos.0).abs()),
        Direction::W => obstacles
            .iter()
            .filter(|(x, y)| *x < start_pos.0 && start_pos.1 == *y)
            .min_by_key(|(x, y)| (*x - start_pos.0).abs()),
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

        let mut places: HashSet<(i32, i32)> = HashSet::new();

        let path = vec![Direction::N, Direction::E, Direction::S, Direction::W];
        let mut current_pos = start_pos;

        places.insert(current_pos);
        let mut directions = path.into_iter().cycle();
        loop {
            if let Some(direction) = directions.next() {
                if let Some(obstacle) = filter_direction(&obstacles, current_pos, direction) {
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
                } else {
                    let tmp_steps = match direction {
                        Direction::N => (current_pos.1 - (0)).abs(),
                        Direction::S => (current_pos.1 - (height - 1)).abs(),
                        Direction::W => (current_pos.0 - (0)).abs(),
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
                    break;
                }
            }
        }

        // let mut map_print = map.clone();
        // places.iter().for_each(|(x, y)| {
        //     let i = x + width * y;
        //     map_print[i as usize] = 'X';
        // });

        // let i = current_pos.0 + width * current_pos.1;
        // map_print[i as usize] = '@';
        // map_print
        //     .chunks(width as usize)
        //     .for_each(|line| println!("{}", line.iter().collect::<String>()));

        self.part_1_result = Some(places.len().to_string());
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
        assert_eq!(result, Some("0".to_string()));
    }

    #[test]
    fn test_day6_part_2_real() {
        let mut puzzle = Day6::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }
}
