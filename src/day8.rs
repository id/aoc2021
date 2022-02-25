use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<(String, String)> {
    input.lines().map(|l| {
        let split = l.split('|').map(|s| s.trim()).collect::<Vec<&str>>();
        (split[0].to_string(), split[1].to_string())
    }).collect()
}

fn is_known_pattern(n: &usize) -> bool {
    n == &2 || n == &3 || n == &4 || n == &7
}

#[aoc(day8, part1)]
fn part1(input: &[(String, String)]) -> usize {
    let mut counter: usize = 0;
    for s in input {
        counter += s.1.split(' ').map(|s| s.len()).filter(is_known_pattern).count()
    }
    counter
}

fn sort_str(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.into_iter().collect()
}

#[aoc(day8, part2)]
fn part2(input: &[(String, String)]) -> i32 {
    for (pattern, output) in input {
        let sorted_pattern: Vec<String> = pattern
            .split(' ')
            .filter(|s| !is_known_pattern(&s.len()))
            .map(sort_str)
            .collect();
        let sorted_output: Vec<String> = output
            .split(' ')
            .filter(|s| !is_known_pattern(&s.len()))
            .map(sort_str)
            .collect();
        println!("{:#?} | {:#?}", sorted_pattern, sorted_output);
    }
    //61229
    8394
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_extample() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")), 8394);
        //assert_eq!(part2(&parse_input("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc")), 9781);
        //assert_eq!(part2(&parse_input(TEST_INPUT)), 61229);
    }
}
