use aoc_runner_derive::{aoc, aoc_generator};
use std::{cmp, cmp::Ordering};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    dx: i32,
    dy: i32,
    len: i32,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        let dx = match x1.cmp(&x2) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1
        };
        let dy = match y1.cmp(&y2) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1
        };
        let len = cmp::max((x1 - x2).abs(), (y1 - y2).abs()) + 1;
        Line { x1, y1, x2, y2, dx, dy, len }
    }
    
    fn horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn vertical(&self) -> bool {
        self.y1 == self.y2
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let coords: Vec<_> = s.split(" -> ")
            .map(|p| p.split(','))
            .flatten()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();
        Line::new(coords[0], coords[1], coords[2], coords[3])
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day5, part1)]
fn part1(lines: &[Line]) -> u32 {
    let mut overlaps = 0;
    let mut coords_map: HashMap<(i32, i32), u32> = HashMap::new();
    for l in lines {
        if l.horizontal() || l.vertical() {
            let mut x = l.x1;
            let mut y = l.y1;
            for _ in 0..l.len {
                *coords_map.entry((x, y)).or_insert(0) += 1;
                if let Some(2) = coords_map.get(&(x, y)) {
                    overlaps += 1;
                }
                x += l.dx;
                y += l.dy;
            }
        }
    }
    overlaps
}

#[aoc(day5, part2)]
fn part2(lines: &[Line]) -> u32 {
    let mut overlaps = 0;
    let mut coords_map: HashMap<(i32, i32), u32> = HashMap::new();
    for l in lines {
        let mut x = l.x1;
        let mut y = l.y1;
        for _ in 0..l.len {
            *coords_map.entry((x, y)).or_insert(0) += 1;
            if let Some(2) = coords_map.get(&(x, y)) {
                overlaps += 1;
            }
            x += l.dx;
            y += l.dy;
        }
    }
    overlaps
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 12);
    }
}
