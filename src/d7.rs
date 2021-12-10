/// AoC 2021 -- Day 7
/// https://adventofcode.com/2021/day/7
use crate::util;

pub fn parse_input(file: &str) -> Vec<u64> {
    let content = util::read_to_string(file).unwrap();
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

/// This is not correct, in general, but is for the two given inputs :)
fn median(xs: &[u64]) -> u64 {
    let mut xs_copy = xs.to_owned();
    xs_copy.sort_unstable();
    let n = xs_copy.len();
    if n % 2 == 0 {
        xs_copy[(n / 2) - 1]
    } else {
        xs_copy[n / 2]
    }
}

/// Solution for part 1 uses the fact that any median of a discrete
/// set of real numbers minimizes the sum of absolute variation.
pub fn d7_part1(input_file: &str) -> u64 {
    let xs = parse_input(input_file);
    let x = median(&xs);
    xs.iter().map(|&n| if x < n { n - x } else { x - n }).sum()
}

/// Naive solution for part 2, uses the identity:
/// 1 + 2 + ... + n = (n + 1) * n / 2
///
/// Performance notes:
///   - runtime is ~880us, including parsing the input
///   - without the "bail early" short circuit, it is ~24% slower
pub fn d7_part2(input_file: &str) -> u64 {
    let xs = parse_input(input_file);
    let xmin = *xs.iter().min().unwrap();
    let xmax = *xs.iter().max().unwrap();
    // start with a safe upper bound
    let mut min_fuel = (xmax + 1) * xmax * (xs.len() as u64);
    for x in xmin..=xmax {
        let mut fuel = 0u64;
        for &n in xs.iter() {
            let d = if x < n { n - x } else { x - n };
            // increment by 1 + 2 + ... + d
            fuel += (d + 1) * d / 2;
            // bail early
            if fuel >= min_fuel {
                break;
            }
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d7_part1() {
        assert_eq!(d7_part1("inputs/d7_test"), 37);
        assert_eq!(d7_part1("inputs/d7"), 352997);
    }

    #[test]
    fn test_d7_part2() {
        assert_eq!(d7_part2("inputs/d7_test"), 168);
        assert_eq!(d7_part2("inputs/d7"), 101571302);
    }
}
