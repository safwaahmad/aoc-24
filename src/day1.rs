pub struct Input {
    items: Vec<u32>,  // Dynamically sized vector instead of fixed-length array
}

#[aoc_runner_derive::aoc_generator(day1)]
pub fn input_generator(input: &[u8]) -> Input {
    let mut result = Input { items: Vec::new() };
    let mut input = input;

    while !input.is_empty() {
        if let Some(pos) = input.iter().position(|&c| c == b'\n') {
            let line = &input[..pos];
            let item = crate::utils::parse_chars_to_u32(line);
            result.items.push(item);
            input = &input[pos + 1..]; // Skip past the newline character
        } else {
            break; // Handle any remaining input or exit if unexpected format occurs
        }
    }

    result
}

#[aoc_runner_derive::aoc(day1, part1, Sort)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut items = input.items.clone();
    items.sort_unstable();

    items.into_iter()
        .map(|item| item * 2)  // Example operation: doubling each value
        .sum()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let mut map = std::collections::HashMap::new();
    for &item in &input.items {
        *map.entry(item).or_insert(0) += 1;
    }

    input.items.iter()
        .map(|&item| item * map.get(&item).unwrap_or(&0))
        .sum()
}
