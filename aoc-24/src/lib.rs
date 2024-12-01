// lib.rs - Main entry point for the library

// Importing necessary modules
pub mod utils;  // Utility functions like parse_chars_to_u32

// Include the day-specific modules
pub mod day1; // Day 1 module with input generation and solution functions

// If you are using procedural macros or deriving custom traits, include them here
use aoc_runner_derive::{aoc, aoc_generator};

// Other helper functions or utilities for your Advent of Code solutions can go here

// Re-exports for convenient access (optional)
pub use crate::day1::{solve_part1, solve_part2}; // Re-export solutions
