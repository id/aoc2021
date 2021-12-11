use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|i| i.parse::<i32>().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[i32]) -> usize {
    let mut fish: Vec<i32> = input.to_owned();
    for _ in 0..80 {
        let len = fish.len();
        for i in 0..len {
            match fish[i] {
                0 => {
                    fish[i] = 6;
                    fish.push(8);
                },
                _ => {
                    fish[i] -= 1;
                }
            }
        }
    }
    fish.len()
}

#[cached]
fn calc_spawn(n: i32, x: i32) -> u64 {
    let initial_spawn = (n + 6 - x) / 7;
    initial_spawn as u64 + (0..initial_spawn).map(|i| calc_spawn(n - 2 - (i+1) * 7, x)).sum::<u64>()
}

#[aoc(day6, part2)]
fn part2(input: &[i32]) -> u64 {
    let mut counter: u64 = 0;
    for x in input {
        counter += calc_spawn(256, *x) + 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input("3,4,3,1,2")), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input("3,4,3,1,2")), 26984457539);
    }
}
