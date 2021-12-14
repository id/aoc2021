use aoc_runner_derive::{aoc, aoc_generator};

const SIZE: usize = 5;
type Order = Vec<u32>;

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<u32>>,
    marked_rows: [usize; SIZE],
    marked_cols: [usize; SIZE],
    marked_total: u32,
    has_won: bool,
}

impl From<&[&str]> for Board {
    fn from(lines: &[&str]) -> Self {
        let mut board: Vec<Vec<u32>> = Vec::new();
        for l in lines {
            board.push(l.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect());
        }
        Self { board, marked_rows: [0; SIZE], marked_cols: [0; SIZE], marked_total: 0, has_won: false }
    }
}

impl Board {
    fn mark(&mut self, n: &u32) -> bool {
        for (i, row) in self.board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col == n {
                    self.marked_rows[i] += 1;
                    self.marked_cols[j] += 1;
                    self.marked_total += self.board[i][j];
                    if self.marked_rows[i] == SIZE || self.marked_cols[j] == SIZE {
                        self.has_won = true;
                        return true;
                    }
                    return false;
                }
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let board_sum = self.board.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
        board_sum - self.marked_total
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Order, Vec<Board>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let order = lines[0].split(',').map(|n| n.parse::<u32>().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut i = 1;
    while i < lines.len() {
        boards.push(lines[i+1..i+SIZE+1].into());
        i += SIZE+1;
    }
    (order, boards)
}

#[aoc(day4, part1)]
fn part1((order, boards): &(Order, Vec<Board>)) -> u32 {
    let mut my_boards = boards.to_vec();
    for i in order {
        for board in my_boards.iter_mut() {
            if board.mark(i) {
                return *i * board.sum_unmarked();
            }
        }
    }
    0
}

#[aoc(day4, part2)]
fn part2((order, boards): &(Order, Vec<Board>)) -> u32 {
    let mut my_boards = boards.to_vec();
    let mut boards_won = 0;
    for i in order {
        for board in my_boards.iter_mut() {
            if board.has_won {
                continue;
            }
            if board.mark(i) {
                boards_won += 1;
                if boards_won == boards.len() {
                    return *i * board.sum_unmarked();
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 4512);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1924);
    }
}
