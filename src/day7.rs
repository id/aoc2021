use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut numbers: Vec<_> = input.to_owned();
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    let pos = numbers[mid];
    numbers.iter().map(|x| (x - pos).abs()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &[i32]) -> i32 {
    let pos = input.iter().sum::<i32>() as f32 / input.len() as f32;
    let answer1 = input.iter().map(|x| (1..=(x - pos.floor() as i32).abs()).sum::<i32>()).sum();
    let answer2 = input.iter().map(|x| (1..=(x - pos.ceil() as i32).abs()).sum::<i32>()).sum();
    std::cmp::min(answer1, answer2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
