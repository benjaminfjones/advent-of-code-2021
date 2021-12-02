/// AoC 2021 -- Day 2
/// https://adventofcode.com/2021/day/2

extern crate aoc_2021;

use aoc_2021::util;

pub struct State {
    aim: i64,
    horiz: i64,
    depth: i64,
}

pub fn d2_driver<F>(input_file: &str, interpreter: F) -> State
    where F: Fn(State, &(&str, i64)) -> State {
    let content = util::read_to_string(input_file).unwrap();
    // parse the input file into a Vec of tuples (command, amount)
    let tuples: Vec<(&str, i64)> = content.trim()
        // make lines
        .split('\n')
        // turn lines -> tuples (&str, i64)
        .map(|s| {
            let splits: Vec<&str> = s.split(" ").collect();
            (splits[0], splits[1].parse::<i64>().unwrap())
        }).collect();
    tuples.iter().fold(State {aim: 0, horiz: 0, depth: 0}, interpreter)
}

pub fn d2_part1_interpreter(st: State, cmd: &(&str, i64)) -> State {
    match *cmd {
        ("forward", x) => State {horiz: st.horiz + x, ..st},
        ("up", x) => State {depth: st.depth - x, ..st},
        ("down", x) => State {depth: st.depth + x, ..st},
        (c, _) => panic!("invalid command: {}", c)
    }
}

pub fn d2_part2_interpreter(st: State, cmd: &(&str, i64)) -> State {
    match *cmd {
        ("forward", x) => State {horiz: st.horiz + x, depth: st.depth + st.aim * x, ..st},
        ("up", x) => State {aim: st.aim - x, ..st},
        ("down", x) => State {aim: st.aim + x, ..st},
        (c, _) => panic!("invalid command: {}", c)
    }
}

pub fn main() {
    let st_part1 = d2_driver("inputs/d2", d2_part1_interpreter);
    println!(
        "horiz {}, depth {}, solution {}",
        st_part1.horiz,
        st_part1.depth,
        st_part1.horiz * st_part1.depth
     );

    let st_part2 = d2_driver("inputs/d2", d2_part2_interpreter);
    println!(
        "horiz {}, depth {}, solution {}",
        st_part2.horiz,
        st_part2.depth,
        st_part2.horiz * st_part2.depth
     );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d2_part1() {
        assert_eq!(d2_driver("inputs/d2_test", d2_part1_interpreter).horiz, 15);
        assert_eq!(d2_driver("inputs/d2_test", d2_part1_interpreter).depth, 10);

        assert_eq!(d2_driver("inputs/d2", d2_part1_interpreter).horiz, 1923);
        assert_eq!(d2_driver("inputs/d2", d2_part1_interpreter).depth, 1001);
    }

    #[test]
    fn test_d2_part2() {
        assert_eq!(d2_driver("inputs/d2_test", d2_part2_interpreter).horiz, 15);
        assert_eq!(d2_driver("inputs/d2_test", d2_part2_interpreter).depth, 60);

        assert_eq!(d2_driver("inputs/d2", d2_part2_interpreter).horiz, 1923);
        assert_eq!(d2_driver("inputs/d2", d2_part2_interpreter).depth, 1030939);
    }
}
