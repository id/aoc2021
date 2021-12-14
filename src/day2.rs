use aoc_runner_derive::{aoc, aoc_generator};

type Move = (i32, i32);

fn parse_line(line: &str) -> Move {
    let pair: Vec<&str> = line.split(' ').collect();
    let d = pair[1].parse().unwrap();
    match pair[0] {
        "forward" => (d, 0),
        "up"      => (0, d),
        "down"    => (0, -d),
        _         => (0, 0)
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Move> {
    input.lines().map(parse_line).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Move]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (dx, dy) in input {
        x += dx;
        y += dy;
    }
    -x*y
}

#[aoc(day2, part2)]
fn part2(input: &[Move]) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for (dx, dy) in input {
        aim -= dy;
        x += dx;
        y += dx * aim;
    }
    x*y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&[(5,0), (0,-5), (8,0), (0,3), (0,-8), (2,0)]),
            150
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[(5,0), (0,-5), (8,0), (0,3), (0,-8), (2,0)]),
            900
        );
    }
}
