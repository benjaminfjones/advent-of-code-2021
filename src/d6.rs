/// AoC 2021 -- Day 6
/// https://adventofcode.com/2021/day/6
///
/// Alternate solution that mutates fish map in-place during simulation.
///
/// Benmark Before (best of 5):
///
/// ```text
/// $ target/release/d6
/// [test] number of fish after 80 days: 5934
/// number of fish after 80 days: 374927 (duration 91.125µs)
/// [test] number of fish after 256 days: 26984457539
/// number of fish after 256 days: 1687617803407 (duration 153.75µs)
/// ```
///
/// Benchmark After (best of 5):
///
/// ```text
/// $ target/release/d6
/// [test] number of fish after 80 days: 5934
/// number of fish after 80 days: 374927 (duration 87.083µs)
/// [test] number of fish after 256 days: 26984457539
/// number of fish after 256 days: 1687617803407 (duration 156.041µs)
/// ```
///
/// There appears to be no performance difference.
///
/// Benchmark after refactoring FishMap to [u64; 9]:
///
/// ```text
/// [test] number of fish after 80 days: 5934
/// number of fish after 80 days: 374927 (duration 37.458µs)
/// [test] number of fish after 256 days: 26984457539
/// number of fish after 256 days: 1687617803407 (duration 20.875µs)
/// ```
///
/// Roughly 8x faster...
use std::time::Instant;

use crate::util;

fn parse_input(file: &str) -> Vec<usize> {
    let content = util::read_to_string(file).unwrap();
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

// Track fish timers using a fixed sized array of ints:
//   * array index = fish timer value
//   * array value = number of fish with a specific timer value
type FishMap = [u64; 9];

/// Add `amt` fish to the map with timer value `timer`
fn inc_fish(fish_map: &mut FishMap, timer: usize, amt: u64) {
    fish_map[timer] += amt;
}

fn dec_fish(fish_map: &mut FishMap, timer: usize, amt: u64) {
    fish_map[timer] -= amt;
}

/// 1 fish, 2 fish, red fish, blue fish
fn count_fish(fish_map: FishMap) -> u64 {
    fish_map.iter().sum()
}

/// Simulate one tick of the clock, mutating the given `fish_map` in-place
fn simulate(fish_map: &mut FishMap) {
    let mut prev_zero_fish: u64 = 0;
    // the only legal timer values are 0..8
    for t in 0..=8 {
        let v = fish_map[t];
        if t == 0 {
            prev_zero_fish = v;
            dec_fish(fish_map, 0, v);
        } else {
            inc_fish(fish_map, t - 1, v);
            dec_fish(fish_map, t, v);
        }
    }
    // add the new spawned fish last so that we don't simulate them in this tick
    // spawn v new fish with timer 8
    inc_fish(fish_map, 8, prev_zero_fish);
    // reset 0 timer fish to 6
    inc_fish(fish_map, 6, prev_zero_fish);
}

fn exec_fish_simulator(input_file: &str, ticks: u64) -> u64 {
    let init_fish_timers = parse_input(input_file);

    let mut fish_map = [0; 9];
    for t in init_fish_timers {
        inc_fish(&mut fish_map, t, 1);
    }

    // simulate fish
    for _ in 1..=ticks {
        simulate(&mut fish_map);
    }
    count_fish(fish_map)
}

fn d6_part1(input_file: &str) -> u64 {
    exec_fish_simulator(input_file, 80)
}

fn d6_part2(input_file: &str) -> u64 {
    exec_fish_simulator(input_file, 256)
}

pub fn main() {
    println!(
        "[test] number of fish after 80 days: {}",
        d6_part1("inputs/d6_test")
    );
    let start = Instant::now();
    let d6_part1_solution = d6_part1("inputs/d6");
    let duration = start.elapsed();
    println!(
        "number of fish after 80 days: {} (duration {:?})",
        d6_part1_solution, duration
    );

    println!(
        "[test] number of fish after 256 days: {}",
        d6_part2("inputs/d6_test")
    );
    let start2 = Instant::now();
    let d6_part2_solution = d6_part2("inputs/d6");
    let duration2 = start2.elapsed();
    println!(
        "number of fish after 256 days: {} (duration {:?})",
        d6_part2_solution, duration2
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d6_part1() {
        assert_eq!(d6_part1("inputs/d6_test"), 5934);
        assert_eq!(d6_part1("inputs/d6"), 374927);
    }

    #[test]
    fn test_d6_part2() {
        assert_eq!(d6_part2("inputs/d6_test"), 26984457539);
        assert_eq!(d6_part2("inputs/d6"), 1687617803407);
    }
}
