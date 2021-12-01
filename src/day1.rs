use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

type Depth = i32;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<Depth>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[Depth]) -> u32 {
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            count += 1
        }
    }
    count
}

#[aoc(day1, part2)]
fn solve_part2(input: &[Depth]) -> u32 {
    let mut count = 0;
    let mut window = input[0] + input[1] + input[2];
    for i in 3..input.len() {
        let next_window = window + input[i] - input[i-3];
        if next_window > window {
            count += 1
        }
        window = next_window
    }
    count
}
