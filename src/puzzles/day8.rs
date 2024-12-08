use std::collections::{HashMap, HashSet};

use crate::adventofcode::Puzzle;

pub struct Day8 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day8 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

fn opposite_direction_points(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    // Calculate the vector between the points
    let vector = (p2.0 - p1.0, p2.1 - p1.1);

    // Calculate new points in the opposite direction
    let new_p1 = (p1.0 - vector.0, p1.1 - vector.1);
    let new_p2 = (p2.0 + vector.0, p2.1 + vector.1);

    (new_p1, new_p2)
}

impl Puzzle for Day8 {
    fn execute_part_1(&mut self, input: &str) {
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;
        let char_map: Vec<char> = input.lines().collect::<String>().chars().collect();
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        let antennas: HashMap<char, Vec<(i32, i32)>> = char_map
            .iter()
            .enumerate()
            .filter(|&c| *c.1 != '.')
            .map(|(i, c)| {
                let x = i as i32 % width;
                let y = i as i32 / width;
                (c, (x, y))
            })
            .fold(HashMap::new(), |mut map, (c, (x, y))| {
                map.entry(*c).or_default().push((x, y));
                map
            });

        for (_c, list) in antennas.iter() {
            for &a in list.iter() {
                for &b in list.iter() {
                    if a == b {
                        continue;
                    }

                    let (p1, p2) = opposite_direction_points(a, b);
                    if p1.0 >= 0 && p1.0 < width && p1.1 >= 0 && p1.1 < height {
                        antinodes.insert(p1);
                    }
                    if p2.0 >= 0 && p2.0 < width && p2.1 >= 0 && p2.1 < height {
                        antinodes.insert(p2);
                    }
                }
            }
        }

        if false {
            // Print the map
            let mut char_map_clone = char_map.clone();
            for (x, y) in antinodes.iter() {
                let i = x + width * y;
                *char_map_clone.get_mut(i as usize).unwrap() = '#';
            }
            char_map_clone
                .chunks(width as usize)
                .for_each(|chunk| println!("{:?}", chunk.iter().collect::<String>()));
        }

        self.part_1_result = Some(antinodes.len().to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let result = 0;
        self.part_2_result = Some(result.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        8
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day8_part_1() {
        let mut puzzle = Day8::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("14".to_string()));
    }

    #[test]
    fn test_day8_part_1_real() {
        let mut puzzle = Day8::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("220".to_string()));
    }

    #[test]
    fn test_day8_part_2() {
        let mut puzzle = Day8::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }

    #[test]
    fn test_day8_part_2_real() {
        let mut puzzle = Day8::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("0".to_string()));
    }
}
