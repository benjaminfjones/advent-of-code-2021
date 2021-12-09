use crate::util;
/// AoC 2021 -- Day 8
/// https://adventofcode.com/2021/day/8
use std::char;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
pub struct Note {
    obs: Vec<String>,
    outputs: Vec<String>,
}

impl FromStr for Note {
    type Err = ();

    /// Notes: observations and outputs are sorted alphabetically in order to
    /// canonicallize the rep'n
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vert_splits = s.split('|');
        let prefix = vert_splits.next().ok_or(())?.trim();
        let suffix = vert_splits.next().ok_or(())?.trim();
        let obs = prefix
            .split(char::is_whitespace)
            .map(|s| normalize(s))
            .collect();
        let outputs = suffix
            .split(char::is_whitespace)
            .map(|s| normalize(s))
            .collect();
        Ok(Note { obs, outputs })
    }
}

fn normalize(s: &str) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    cs.sort();
    cs.into_iter().collect()
}

pub fn parse_input(input_file: &str) -> Vec<Note> {
    let content = util::read_to_string(input_file).unwrap();
    content
        .trim()
        .split('\n')
        .map(|s| s.parse::<Note>().expect("failed to parse Note from input"))
        .collect()
}

fn is_uniq_output(output: &str) -> bool {
    let n = output.len();
    n == 2 || n == 3 || n == 4 || n == 7
}

pub fn d8_part1(notes: &Vec<Note>) -> usize {
    let mut count: usize = 0;
    for n in notes {
        for out in n.outputs.iter() {
            if is_uniq_output(out) {
                count += 1;
            }
        }
    }
    count
}

type SignalSet = String;

/// Return the number of signal wires common to both sets
fn num_common_signals(ss1: &SignalSet, ss2: &SignalSet) -> usize {
    let hss1: HashSet<char> = ss1.chars().collect();
    let hss2: HashSet<char> = ss2.chars().collect();
    hss1.intersection(&hss2).collect::<HashSet<&char>>().len()
}

/// Add a signal set -> digit inference fact
fn infer(
    sig_map: &mut HashMap<SignalSet, u32>,
    digit_map: &mut HashMap<u32, SignalSet>,
    signal: &SignalSet,
    digit: u32,
) {
    sig_map.insert(signal.clone(), digit);
    digit_map.insert(digit, signal.clone());
}

/// Infer a signal map from the observations in the given Note
///
/// TODO: refactor to more of a horn-clause saturation approach
///
/// Segment display for reference:
///   0:      1:      2:      3:      4:
///  aaaa    ....    aaaa    aaaa    ....
/// b    c  .    c  .    c  .    c  b    c
/// b    c  .    c  .    c  .    c  b    c
///  ....    ....    dddd    dddd    dddd
/// e    f  .    f  e    .  .    f  .    f
/// e    f  .    f  e    .  .    f  .    f
///  gggg    ....    gggg    gggg    ....
///
///   5:      6:      7:      8:      9:
///  aaaa    aaaa    aaaa    aaaa    aaaa
/// b    .  b    .  .    c  b    c  b    c
/// b    .  b    .  .    c  b    c  b    c
///  dddd    dddd    ....    dddd    dddd
/// .    f  e    f  .    f  e    f  .    f
/// .    f  e    f  .    f  e    f  .    f
///  gggg    gggg    ....    gggg    gggg
///
fn infer_signal_mapping(note: &Note) -> HashMap<SignalSet, u32> {
    let mut sig_map: HashMap<SignalSet, u32> = HashMap::new();
    let mut digit_map: HashMap<u32, SignalSet> = HashMap::new();
    let obs_sets: Vec<SignalSet> = note
        .obs
        .iter()
        .map(|s| s.chars().collect::<SignalSet>())
        .collect();

    // first pass: collect unique signal patterns
    for obs in obs_sets.iter() {
        let n = obs.len();
        match n {
            2 => {
                infer(&mut sig_map, &mut digit_map, obs, 1);
            }
            3 => {
                infer(&mut sig_map, &mut digit_map, obs, 7);
            }
            4 => {
                infer(&mut sig_map, &mut digit_map, obs, 4);
            }
            7 => {
                infer(&mut sig_map, &mut digit_map, obs, 8);
            }
            _ => {
                continue;
            }
        }
    }
    let sig_1 = digit_map
        .get(&1)
        .expect("failed to find signal for 1")
        .clone();
    let sig_4 = digit_map
        .get(&4)
        .expect("failed to find signal for 1")
        .clone();

    // second pass: infer signal patterns of length 5, 6
    for obs in obs_sets.iter() {
        let n = obs.len();
        let common_with_1 = num_common_signals(obs, &sig_1);
        let common_with_4 = num_common_signals(obs, &sig_4);
        match n {
            // digit is in {2, 3, 5}
            // obs /\ 1 == 2 ==> infer 3
            // obs /\ 4 == 2 ==> infer 2
            // obs /\ 4 == 3 ==> infer 3, 5
            5 => {
                if common_with_1 == 2 {
                    infer(&mut sig_map, &mut digit_map, obs, 3);
                } else if common_with_4 == 2 {
                    infer(&mut sig_map, &mut digit_map, obs, 2);
                } else if common_with_4 == 3 {
                    infer(&mut sig_map, &mut digit_map, obs, 5);
                } else {
                    panic!("5-signal inference failed for: {}", obs);
                }
            }

            // digit is in {0, 6, 9}
            // obs /\ 1 == 1 ==> infer 6
            // obs /\ 4 == 3 ==> 0, 6
            // obs /\ 4 == 4 ==> 9
            6 => {
                if common_with_1 == 1 {
                    infer(&mut sig_map, &mut digit_map, obs, 6);
                } else if common_with_4 == 3 {
                    infer(&mut sig_map, &mut digit_map, obs, 0);
                } else if common_with_4 == 4 {
                    infer(&mut sig_map, &mut digit_map, obs, 9);
                } else {
                    panic!(
                        "5-signal inference failed for: {}, digit_map: {:?}",
                        obs, digit_map
                    );
                }
            }

            _ => {
                continue;
            }
        }
    }

    if digit_map.len() != 10 {
        panic!("could not infer all digits, digit_map: {:?}", digit_map);
    }
    sig_map
}

fn decode_outputs(sig_map: &HashMap<SignalSet, u32>, outputs: &Vec<String>) -> u32 {
    let mut result = 0u32;
    for out_sig in outputs {
        let digit = sig_map.get(out_sig).expect(&format!(
            "could not find pattern: {} in signal map: {:?}",
            out_sig, sig_map
        ));
        result = result * 10 + digit;
    }
    result
}

pub fn d8_part2(notes: &Vec<Note>) -> u32 {
    let mut solution = 0u32;
    for n in notes.iter() {
        let sig_map = infer_signal_mapping(n);
        let decoded_output = decode_outputs(&sig_map, &n.outputs);
        solution += decoded_output;
    }
    solution
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_input_d8_test() {
        let notes = parse_input("inputs/d8_test");
        assert_eq!(notes.len(), 10);
    }

    #[test]
    fn test_parser_input_d8() {
        let notes = parse_input("inputs/d8");
        assert_eq!(notes.len(), 200);
    }

    #[test]
    fn test_d8_part1_test() {
        let notes = parse_input("inputs/d8_test");
        assert_eq!(d8_part1(&notes), 26);
    }

    #[test]
    fn test_d8_part1() {
        let notes = parse_input("inputs/d8");
        assert_eq!(d8_part1(&notes), 416);
    }

    #[test]
    fn test_infer_example() {
        let test_input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let expected_mapping: HashMap<String, u32> = [
            ("acedgfb", 8),
            ("cdfbe", 5),
            ("gcdfa", 2),
            ("fbcad", 3),
            ("dab", 7),
            ("cefabd", 9),
            ("cdfgeb", 6),
            ("eafb", 4),
            ("cagedb", 0),
            ("ab", 1),
        ]
        .iter()
        .map(|(s, n)| (normalize(s), *n as u32))
        .collect();

        let note = test_input.parse::<Note>().unwrap();
        let sig_map = infer_signal_mapping(&note);
        assert_eq!(sig_map, expected_mapping);

        assert_eq!(decode_outputs(&sig_map, &note.outputs), 5353);
    }

    #[test]
    fn test_d8_part2_test() {
        let notes = parse_input("inputs/d8_test");
        assert_eq!(d8_part2(&notes), 61229);
    }

    #[test]
    fn test_d8_part2() {
        let notes = parse_input("inputs/d8");
        assert_eq!(d8_part2(&notes), 1043697);
    }
}
