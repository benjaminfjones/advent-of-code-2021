/// AoC 2021 -- Day 10
/// https://adventofcode.com/2021/day/10
use crate::util;

type Line = Vec<char>;
type Stack = Vec<char>;
type Score = u64;

pub fn parse_input(input_file: &str) -> Vec<Line> {
    let content = util::read_to_string(input_file).unwrap();
    content
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect()
}

fn is_chunk_start(c: char) -> bool {
    matches!(c, '(' | '[' | '<' | '{')
}

fn chunk_match(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        _ => panic!("woah! bad chunk dude"),
    }
}

fn part1_score(c: char) -> Score {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("woah! bad chunk-end dude"),
    }
}

fn part2_score(c: char) -> Score {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("woah! bad chunk-end dude"),
    }
}

/// If line is corrupt, return Some(first invalid chunk closer) and the remaining stack, if not
/// corrupt, return None
pub fn corrupt_line(line: Line) -> (Option<char>, Stack) {
    let mut stack: Stack = Vec::new();
    for c in line {
        if is_chunk_start(c) {
            stack.push(c);
        } else {
            let top = stack[stack.len() - 1];
            if c == chunk_match(top) {
                stack.pop();
            } else {
                // invalid chunk-end
                // return the chunk-end we expected to find
                return (Some(c), stack);
            }
        }
    }
    (None, stack)
}

pub fn d10_part1(lines: Vec<Line>) -> Score {
    lines
        .into_iter()
        .map(|l| corrupt_line(l).0)
        .flatten()
        .map(part1_score)
        .sum()
}

pub fn d10_part2(lines: Vec<Line>) -> Score {
    let mut lc_scores: Vec<Score> = lines
        .into_iter()
        .map(corrupt_line)
        // filter out corrupted lines
        .filter(|(c, _)| c.is_none())
        // select just the remaining stack, reverse each chunk delimiter, then reverse the stack
        .map(|(_, stack)| {
            stack
                .into_iter()
                .map(chunk_match)
                .rev()
                .fold(0, |acc, c| 5 * acc + part2_score(c))
        })
        .collect();
    lc_scores.sort_unstable();
    // return the middle score, assuming lc_scores is always odd
    assert!(lc_scores.len() % 2 == 1);
    lc_scores[lc_scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let test_lines = parse_input("inputs/d10_test");
        assert_eq!(test_lines.len(), 10);
        let lines = parse_input("inputs/d10");
        assert_eq!(lines.len(), 106);
    }

    #[test]
    fn test_corrupt_line_ex() {
        let line1: Line = "{([(<{}[<>[]}>{[]{[(<()>".chars().collect();
        assert_eq!(corrupt_line(line1).0, Some('}'));
        let line2: Line = "[[<[([]))<([[{}[[()]]]".chars().collect();
        assert_eq!(corrupt_line(line2).0, Some(')'));
        let line3: Line = "[[<[([])<([[{}[[()]]]".chars().collect();
        assert_eq!(corrupt_line(line3).0, None);
    }

    #[test]
    fn test_d10_part1_test() {
        let test_lines = parse_input("inputs/d10_test");
        assert_eq!(d10_part1(test_lines), 26397);
    }

    #[test]
    fn test_d10_part1() {
        let lines = parse_input("inputs/d10");
        assert_eq!(d10_part1(lines), 411471);
    }

    #[test]
    fn test_d10_part2_test() {
        let test_lines = parse_input("inputs/d10_test");
        assert_eq!(d10_part2(test_lines), 288957);
    }

    #[test]
    fn test_d10_part2() {
        let lines = parse_input("inputs/d10");
        assert_eq!(d10_part2(lines), 3122628974);
    }
}
