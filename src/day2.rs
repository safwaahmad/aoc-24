use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe_report(report: &[i32]) -> bool {
    // Compute the differences between consecutive elements
    let differences: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    // Check if all differences are positive (increasing)
    let is_increasing = differences.iter().all(|&d| d > 0);
    // Check if all differences are negative (decreasing)
    let is_decreasing = differences.iter().all(|&d| d < 0);
    // Check if all differences are within the range [1, 3]
    let are_differences_valid = differences.iter().all(|&d| (1..=3).contains(&d.abs()));

    // A report is safe if it's either increasing or decreasing and the differences are valid
    (is_increasing || is_decreasing) && are_differences_valid
}

fn main() -> io::Result<()> {
    // Specify the input file path
    let input_file_path = "input.txt";

    // Open the file
    let file = File::open(input_file_path)?;
    let reader = io::BufReader::new(file);

    // Parse the input into a vector of vectors
    let reports: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();

    // Count the number of safe reports
    let safe_count = reports.iter().filter(|&report| is_safe_report(report)).count();

    // Print the result
    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
