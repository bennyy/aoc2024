use std::{cmp::Ordering, collections::HashMap};

use crate::adventofcode::Puzzle;

pub struct Day5 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
    page_ordering_rules: HashMap<u32, Vec<u32>>,
    rev_page_ordering_rules: HashMap<u32, Vec<u32>>,
    page_numbers_list: Vec<Vec<u32>>,
}

impl Day5 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
            page_ordering_rules: HashMap::new(),
            rev_page_ordering_rules: HashMap::new(),
            page_numbers_list: Vec::new(),
        }
    }

    fn sort_pages(&self, next_page: &u32, current_page: &u32) -> Ordering {
        // Check if the current page has a page ordering rule
        if let Some(page_ordering_rule) = self.page_ordering_rules.get(current_page) {
            if page_ordering_rule.contains(next_page) {
                // Current page has a page ordering rule
                return std::cmp::Ordering::Equal;
            }
        }

        // If not, check what other page points to the current page
        if let Some(what_points_to_this) = self.rev_page_ordering_rules.get(current_page) {
            if what_points_to_this.contains(next_page) {
                // Yes, it points to the current page therefore we need to switch them.
                return std::cmp::Ordering::Less;
            }
        }

        panic!("This should not happen?");
    }

    fn init_rules(&mut self, input: &str) {
        self.page_ordering_rules = input
            .lines()
            .filter_map(|line| line.split_once('|'))
            .map(|(left, right)| {
                let left = left.trim().parse::<u32>().unwrap();
                let right = right.trim().parse::<u32>().unwrap();
                (left, right)
            })
            .fold(HashMap::new(), |mut map, (left, right)| {
                map.entry(left).or_default().push(right);
                map
            });

        self.rev_page_ordering_rules = input
            .lines()
            .filter_map(|line| line.split_once('|'))
            .map(|(left, right)| {
                let left = left.trim().parse::<u32>().unwrap();
                let right = right.trim().parse::<u32>().unwrap();
                (left, right)
            })
            .fold(HashMap::new(), |mut map, (left, right)| {
                map.entry(right).or_default().push(left);
                map
            });

        self.page_numbers_list = input
            .lines()
            .filter(|line| line.contains(','))
            .map(|line| {
                line.split(',')
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
    }
}

impl Puzzle for Day5 {
    fn execute_part_1(&mut self, input: &str) {
        self.init_rules(input);

        let mut result = 0;
        for page_numbers in &self.page_numbers_list {
            let mut page_numbers_copy = page_numbers.clone();
            page_numbers_copy
                .sort_by(|next_page, current_page| self.sort_pages(next_page, current_page));

            if page_numbers == &page_numbers_copy {
                result += page_numbers.get((page_numbers.len() - 1) / 2).unwrap();
            }
        }

        self.part_1_result = Some(result.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        self.init_rules(input);

        let mut result = 0;
        for page_numbers in &self.page_numbers_list {
            let mut page_numbers_copy = page_numbers.clone();
            page_numbers_copy
                .sort_by(|next_page, current_page| self.sort_pages(next_page, current_page));

            if page_numbers != &page_numbers_copy {
                result += page_numbers_copy
                    .get((page_numbers_copy.len() - 1) / 2)
                    .unwrap();
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
        5
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day5_part_1() {
        let mut puzzle = Day5::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("143".to_string()));
    }

    #[test]
    fn test_day5_part_1_real() {
        let mut puzzle = Day5::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("4774".to_string()));
    }

    #[test]
    fn test_day5_part_2() {
        let mut puzzle = Day5::new();
        let input = read_test_input(puzzle.day(), None, None);

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("123".to_string()));
    }

    #[test]
    fn test_day5_part_2_real() {
        let mut puzzle = Day5::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("6004".to_string()));
    }
}
