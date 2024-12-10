use std::{
    cmp::Ordering,
    collections::{BTreeMap, VecDeque},
};

use crate::adventofcode::Puzzle;

pub struct Day9 {
    part_1_result: Option<String>,
    part_2_result: Option<String>,
}

impl Day9 {
    pub fn new() -> Self {
        Self {
            part_1_result: None,
            part_2_result: None,
        }
    }
}

type FileId = usize;
type FileSize = u32;

impl Puzzle for Day9 {
    fn execute_part_1(&mut self, input: &str) {
        let mut files: Vec<(usize, Option<FileId>, FileSize)> = input
            .trim()
            .chars()
            .enumerate() // Add the original index
            .filter(|(idx, _)| idx % 2 == 0) // Keep only every second character
            .enumerate() // Add the filtered index
            .map(|(file_id, (original_idx, c))| {
                (original_idx, Some(file_id), c.to_digit(10).unwrap())
            })
            .collect();

        let mut free_space: VecDeque<(usize, FileSize)> = input
            .trim()
            .chars()
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0) // Keep odd-indexed chars
            .map(|(index, c)| (index, c.to_digit(10).unwrap()))
            .collect();

        let mut files_filled_in_freed_area: Vec<(usize, Option<FileId>, FileSize)> = Vec::new();
        for (file_index, file_id, file_size) in files.iter_mut().rev() {
            while *file_size != 0 {
                if let Some(current_free_block) = free_space.pop_front() {
                    let (free_index, mut free_size) = current_free_block;

                    if free_index > *file_index {
                        // Only handle freed indexes after the current file
                        break;
                    }

                    match file_size.cmp(&&mut free_size) {
                        Ordering::Greater => {
                            // The file is larger than this free space can hold.

                            // Done! Push this new (prev freed) block into a separate list.
                            files_filled_in_freed_area.push((free_index, *file_id, free_size));

                            *file_size -= free_size; // Update the current file with the remaning size.
                        }
                        Ordering::Equal => {
                            // Perfect fit!

                            // Done! Push this new (prev freed) block into a separate list.
                            files_filled_in_freed_area.push((free_index, *file_id, *file_size));

                            // Size 0 means this block is no longer applicable.
                            *file_size = 0;
                        }
                        Ordering::Less => {
                            // The smaller is larger than this free space can hold.
                            // So take a chunk of it and return the free block to the beginning of the list.

                            let available_size_left = free_size - *file_size;

                            // Split the free space into two chunks.

                            // First, push this new (prev freed) block into a separate list.
                            files_filled_in_freed_area.push((free_index, *file_id, *file_size));

                            // Second, give back the free block into the beginning of the queue
                            free_space.push_front((free_index, available_size_left));

                            // Size 0 means this block is no longer applicable.
                            *file_size = 0;
                        }
                    }
                } else {
                    // No more free spaces
                    break;
                }
            }
        }

        let highest_index = std::cmp::max(
            files.last().unwrap().0,
            files_filled_in_freed_area.last().unwrap().0,
        );

        let mut disk_map = Vec::new();
        for file_index in 0..highest_index {
            let relevant_files: Vec<_> = files
                .iter()
                .chain(&files_filled_in_freed_area)
                .filter(|&&(i, _, _)| i == file_index)
                .collect();

            for (_index, id, size) in relevant_files.iter() {
                if let Some(id) = id {
                    for value in std::iter::repeat(id).take(*size as usize) {
                        disk_map.push(*value);
                    }
                }
            }
        }

        let res: usize = disk_map
            .iter()
            .enumerate()
            .map(|(index, value)| index * value)
            .sum();

        self.part_1_result = Some(res.to_string());
    }

    fn execute_part_2(&mut self, input: &str) {
        let files: Vec<(usize, Option<FileId>, FileSize)> = input
            .trim()
            .chars()
            .enumerate() // Add the original index
            .filter(|(idx, _)| idx % 2 == 0) // Keep only every second character
            .enumerate() // Add the filtered index
            .map(|(file_id, (original_idx, c))| {
                (original_idx, Some(file_id), c.to_digit(10).unwrap())
            })
            .collect();

        let free_space: VecDeque<(usize, Option<FileId>, FileSize)> = input
            .trim()
            .chars()
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0) // Keep odd-indexed chars
            .map(|(index, c)| (index, None, c.to_digit(10).unwrap()))
            .collect();

        let mut disk_map_map: BTreeMap<usize, Vec<(Option<FileId>, FileSize)>> = BTreeMap::new();
        for (idx, file_id, file_size) in files.iter() {
            disk_map_map
                .entry(*idx)
                .or_insert_with(Vec::new)
                .push((*file_id, *file_size));
        }

        for (idx, _, free_space) in free_space.iter() {
            disk_map_map
                .entry(*idx)
                .or_insert_with(Vec::new)
                .push((None, *free_space));
        }

        for (file_index, file_id, file_size) in files.iter().rev() {
            let next_first_free_space = disk_map_map
                .iter_mut()
                .filter(|(index, blocks)| {
                    blocks
                        .iter()
                        .filter(|(id, size)| {
                            id.is_none() && size >= file_size && index < &file_index
                        })
                        .count()
                        != 0
                })
                .min_by_key(|(index, _)| **index);

            let index_to_remove;
            if let Some((index, blocks)) = next_first_free_space {
                if index >= file_index {
                    break;
                }

                if let Some((free_block_id, free_block_size)) = blocks
                    .iter_mut()
                    .find(|(free_block_id, _)| free_block_id.is_none())
                {
                    match file_size.cmp(free_block_size) {
                        Ordering::Greater => continue,
                        Ordering::Equal => {
                            *free_block_id = *file_id;
                            *free_block_size = *file_size;

                            index_to_remove = Some((*file_index, file_id.unwrap()));
                        }
                        Ordering::Less => {
                            let available_size_left = *free_block_size - *file_size;

                            *free_block_id = *file_id;
                            *free_block_size = *file_size;

                            blocks.push((None, available_size_left));

                            index_to_remove = Some((*file_index, file_id.unwrap()));
                        }
                    }
                    if let Some((index, file_id_to_be_removed)) = index_to_remove {
                        let blocks = disk_map_map.get_mut(&index);
                        if let Some(blocks) = blocks {
                            blocks
                                .iter_mut()
                                .filter(|(file_id, _)| {
                                    file_id.is_some() && file_id.unwrap() == file_id_to_be_removed
                                })
                                .for_each(|(file_id, _)| {
                                    *file_id = None;
                                });
                        }
                    }
                }
            }
        }

        let res: usize = disk_map_map
            .iter()
            .flat_map(|(_, list)| {
                list.iter().flat_map(|(file_id, file_size)| {
                    std::iter::repeat(file_id.unwrap_or(0)).take(*file_size as usize)
                })
            })
            .enumerate()
            .map(|(index, value)| index * value)
            .sum();

        self.part_2_result = Some(res.to_string());
    }

    fn get_result_part_1(&self) -> Option<String> {
        self.part_1_result.clone()
    }

    fn get_result_part_2(&self) -> Option<String> {
        self.part_2_result.clone()
    }

    fn day(&self) -> u32 {
        9
    }
}

#[cfg(test)]
mod tests {
    use crate::filehelper::{read_input, read_test_input};

    use super::*;

    #[test]
    fn test_day9_part_1() {
        let mut puzzle = Day9::new();
        let input = read_test_input(puzzle.day(), None, Some(2));

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("1928".to_string()));
    }

    #[test]
    fn test_day9_part_1_real() {
        let mut puzzle = Day9::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_1(&input);

        let result = puzzle.get_result_part_1();
        assert_eq!(result, Some("6390180901651".to_string()));
    }

    #[test]
    fn test_day9_part_2() {
        let mut puzzle = Day9::new();
        let input = read_test_input(puzzle.day(), None, Some(2));

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("2858".to_string()));
    }

    #[test]
    fn test_day9_part_2_real() {
        let mut puzzle = Day9::new();
        let input = read_input(puzzle.day());

        puzzle.execute_part_2(&input);

        let result = puzzle.get_result_part_2();
        assert_eq!(result, Some("6412390114238".to_string()));
    }
}
