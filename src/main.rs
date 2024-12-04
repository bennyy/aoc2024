mod adventofcode;
mod filehelper;
mod puzzles;

use std::time::Instant;

use adventofcode::AdventOfCode;
use filehelper::read_input;

fn main() {
    let total_days = 25;
    let mut aoc = AdventOfCode::new();

    // Add a puzzles
    aoc.add_puzzle(1, Box::new(puzzles::day1::Day1::new()));
    aoc.add_puzzle(2, Box::new(puzzles::day2::Day2::new()));
    aoc.add_puzzle(3, Box::new(puzzles::day3::Day3::new()));
    aoc.add_puzzle(4, Box::new(puzzles::day4::Day4::new()));

    // Define widths for table columns
    let day_width = 5;
    let result_width = 30;
    let time_width = 8;
    let result_time_width = result_width + time_width + 1;

    // Print the header of the table
    println!(
        "{:day_width$}  {:result_time_width$} | {:result_time_width$}",
        "".to_string(),
        "Part 1".to_string(),
        "Part 2".to_string()
    );
    println!(
        "{:>day_width$}  {:<result_width$} {:<time_width$} | {:<result_width$} {:<time_width$}",
        "Day".to_string(),
        "Result".to_string(),
        "Time".to_string(),
        "Result".to_string(),
        "Time".to_string()
    );
    println!(
        "{:day_width$}",
        "-".repeat(result_time_width * 2 + day_width + 10)
    );

    // Loop through all days and execute puzzles
    for day in 1..=total_days {
        if let Some(_puzzle) = aoc.puzzles.get_mut(&day) {
            let puzzle = aoc.get_puzzle(day);

            // Get the day's input as a string
            let input = read_input(puzzle.day());

            // Measure execution time for Part 1
            let start_part_1 = Instant::now();
            puzzle.execute_part_1(&input);
            let duration_part_1 = start_part_1.elapsed();
            let part_1_result = puzzle
                .get_result_part_1()
                .unwrap_or("Not solved".to_string());

            // Measure execution time for Part 2
            let start_part_2 = Instant::now();
            puzzle.execute_part_2(&input);
            let duration_part_2 = start_part_2.elapsed();
            let part_2_result = puzzle
                .get_result_part_2()
                .unwrap_or("Not solved".to_string());

            // Print results for the day
            println!(
                "{:>day_width$}  {:<result_width$} {:<time_width$} | {:<result_width$} {:<time_width$}",
                day,
                part_1_result,
                format!("{}ms", duration_part_1.as_millis()),
                part_2_result,
                format!("{}ms", duration_part_2.as_millis())
            );
        } else {
            // Print placeholder results for unimplemented puzzles
            println!(
                "{:>day_width$}  {:<result_width$} {:<time_width$} | {:<result_width$} {:<time_width$}",
                day,
                "".to_string(),
                0,
                "".to_string(),
                0
            );
        }
    }
}
