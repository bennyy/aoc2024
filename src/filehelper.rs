use std::fs;
use std::path::Path;

/// Reads the test input file for a given day and part of the puzzle.
///
/// # Arguments
///
/// * `day` - The day of the puzzle.
/// * `part` - The part of the puzzle (1 or 2).
/// * `file_number` - An optional file number to distinguish between multiple test files.
///
/// # Returns
///
/// A `String` containing the contents of the test input file.
///
/// # Panics
///
/// Panics if the input file cannot be read.
#[cfg(test)]
pub fn read_test_input(day: u32, part: u32, file_number: Option<u32>) -> String {
    let file_name = if let Some(number) = file_number {
        format!("test_day_{:02}_part{}_{}.txt", day, part, number)
    } else {
        format!("test_day_{:02}_part{}.txt", day, part)
    };
    let path = Path::new("inputs").join(file_name);

    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read input file '{}': {}", path.display(), err))
}

/// Reads the input file for a given day of the puzzle.
///
/// # Arguments
///
/// * `day` - The day of the puzzle.
///
/// # Returns
///
/// A `String` containing the contents of the input file.
///
/// # Panics
///
/// Panics if the input file cannot be read.
pub fn read_input(day: u32) -> String {
    let file_name = format!("day_{:02}.txt", day);
    let path = Path::new("inputs").join(file_name);

    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read input file '{}': {}", path.display(), err))
}
