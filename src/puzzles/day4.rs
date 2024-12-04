use crate::adventofcode::Puzzle;

pub struct Day4 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day4 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    SE,
    NW,
    SW,
}

impl Direction {
    fn coordinates(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
            Direction::NE => (1, -1),
            Direction::SE => (1, 1),
            Direction::NW => (-1, -1),
            Direction::SW => (-1, 1),
        }
    }
}

static DIRECTION_ITER: [Direction; 8] = [
    Direction::N,
    Direction::S,
    Direction::W,
    Direction::E,
    Direction::NE,
    Direction::SE,
    Direction::NW,
    Direction::SW,
];

fn get_indexes(
    direction: &Direction,
    steps: i32,
    start_x: i32,
    start_y: i32,
    max_x: i32,
    max_y: i32,
) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();

    let end_x = start_x + direction.coordinates().0 * steps;
    let end_y = start_y + direction.coordinates().1 * steps;
    if (end_x >= 0 && end_x < max_x) && (end_y >= 0 && end_y < max_y) {
    } else {
        return indexes;
    }

    for i in 0..=steps {
        let x = start_x + direction.coordinates().0 * i;
        let y = start_y + direction.coordinates().1 * i;
        indexes.push(x as usize + max_x as usize * y as usize);
    }

    indexes
}

impl Puzzle for Day4 {
    fn execute_part_1(&mut self, input: &str) {
        let lines_str = input.split('\n');

        let height = lines_str.clone().count() as i32;
        let width = lines_str.clone().next().unwrap().len() as i32;
        let map: Vec<char> = input.lines().collect::<String>().chars().collect();

        let xmas = "XMAS";
        let steps: i32 = xmas.len() as i32 - 1;

        let mut result = 0;
        for (i, c) in map.iter().enumerate() {
            match c {
                'X' => {
                    let i = i as i32;
                    let x = i % width;
                    let y = i / width;
                    for direction in DIRECTION_ITER.iter() {
                        let indexes = get_indexes(direction, steps, x, y, width, height);

                        let has_xmas = indexes
                            .iter()
                            .map(|&index| *map.get(index).unwrap())
                            .eq(['X', 'M', 'A', 'S']);

                        if has_xmas {
                            result += 1;
                        }
                    }
                }
                _ => continue,
            };
        }
        self.part_1_result = Some(result.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let lines_str = input.split('\n');
        let height = lines_str.clone().count() as i32;
        let width = lines_str.clone().next().unwrap().len() as i32;

        let map: Vec<char> = input.lines().collect::<String>().chars().collect();

        let mut result = 0;
        for (i, c) in map.iter().enumerate() {
            match c {
                'A' => {
                    let i = i as i32;
                    let x = i % width;
                    let y = i / width;

                    let nw = get_indexes(&Direction::NW, 1, x, y, width, height);
                    let se = get_indexes(&Direction::SE, 1, x, y, width, height);
                    let ne = get_indexes(&Direction::NE, 1, x, y, width, height);
                    let sw = get_indexes(&Direction::SW, 1, x, y, width, height);

                    if !nw.is_empty() && !se.is_empty() && !nw.is_empty() && !se.is_empty() {
                        let a1 = *map.get(*nw.last().unwrap()).unwrap();
                        let b1 = *map.get(*se.last().unwrap()).unwrap();

                        let a2 = *map.get(*ne.last().unwrap()).unwrap();
                        let b2 = *map.get(*sw.last().unwrap()).unwrap();

                        if ((a1 == 'M' && b1 == 'S') || (a1 == 'S' && b1 == 'M'))
                            && ((a2 == 'M' && b2 == 'S') || (a2 == 'S' && b2 == 'M'))
                        {
                            result += 1;
                        }
                    }
                }
                _ => continue,
            };
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
        4
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day4_part_1() {
        let mut puzzle = Day4::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("18".to_string()));
    }

    #[test]
    fn test_day4_part_1_real() {
        let mut puzzle = Day4::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("2573".to_string()));
    }

    #[test]
    fn test_day4_part_2() {
        let mut puzzle = Day4::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("9".to_string()));
    }

    #[test]
    fn test_day4_part_2_real() {
        let mut puzzle = Day4::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("1850".to_string()));
    }
}
