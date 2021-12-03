use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (usize, Vec<i32>) {
    let len = input.find('\n').unwrap();
    (len, input.lines().map(|l| isize::from_str_radix(l, 2).unwrap() as i32).collect())
}

fn calc_set_bits_count(len: usize, input: &Vec<i32>) -> Vec<i32> {
    let mut set_bits_count = vec![0; len];
    for n in input {
        for i in 0..len {
            set_bits_count[len-i-1] += (n >> i) & 1;
        }
    }
    set_bits_count
}

#[aoc(day3, part1)]
fn part1((len, input): &(usize, Vec<i32>)) -> i32 {
    let set_bits_count = calc_set_bits_count(*len, input);
    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = (input.len() as i32) / 2;
    for i in 0..*len {
        if set_bits_count[i] >= threshold {
            gamma |= 1 << (*len-i-1);
        }
        else {
            epsilon |= 1 << (*len-i-1);
        }
    }
    gamma * epsilon
}

fn ge(left: i32, right: i32) -> bool {
    left >= right
}

fn lt(left: i32, right: i32) -> bool {
    left < right
}

fn calc_rating(len: usize, input: &Vec<i32>, cmp: &dyn Fn(i32, i32) -> bool) -> i32 {
    let mut ratings = input.clone();
    let mut res = 0;
    for i in 0..len {
        let set_bits_count = calc_set_bits_count(len, &ratings);
        let threshold = ratings.len() as i32 - set_bits_count[i];
        let bit_criteria = if cmp(set_bits_count[i], threshold) { 1 } else { 0 };
        ratings.retain(|&x| (x >> (len - i - 1)) & 1 == bit_criteria);
        if ratings.len() == 1 {
            res = ratings[0];
            break;
        }
    }
    res
}

#[aoc(day3, part2)]
fn part2((len, input): &(usize, Vec<i32>)) -> i32 {
    let o2 = calc_rating(*len, input, &ge);
    let co2 = calc_rating(*len, input, &lt);
    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 230);
    }
}
