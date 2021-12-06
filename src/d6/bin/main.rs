/// AoC 2021 -- Day 6
/// https://adventofcode.com/2021/day/6

extern crate aoc_2021;

use std::collections::HashMap;

use aoc_2021::util;

fn parse_input(file: &str) -> Vec<u64> {
    let content = util::read_to_string(file).unwrap();
    content.trim().split(',').map(|s| s.parse::<u64>().unwrap()).collect()
}

// map from fish timer value -> # of isomorphic fish with that timer
type FishMap = HashMap<u64, u64>;

/// Add `amt` fish to the map with timer value `timer`
fn inc_fish(fish_map: &mut FishMap, timer: u64, amt: u64) {
    if let Some(&c) = fish_map.get(&timer) {
        fish_map.insert(timer, c+amt);
    } else {
        fish_map.insert(timer, amt);
    }
}

/// 1 fish, 2 fish, red fish, blue fish
fn count_fish(fish_map: FishMap) -> u64 {
    fish_map.values().sum()
}

/// Simulate one tick of the clock, producing a new fish map.
fn simulate(fish_map: FishMap) -> FishMap {
    let mut new_fish_map: FishMap = HashMap::new();
    for (&t, &v) in fish_map.iter() {
        if t == 0 {
            // spawn v new fish with timer 8
            inc_fish(&mut new_fish_map, 8, v);
            // reset 0 timer fish to 6
            inc_fish(&mut new_fish_map, 6, v);
        } else {
            inc_fish(&mut new_fish_map, t-1, v);
        }
    }
    new_fish_map
}

fn exec_fish_simulator(input_file: &str, ticks: u64) -> u64 {
    let init_fish_timers = parse_input(input_file);

    let mut fish_map: HashMap<u64, u64> = HashMap::new();
    for t in init_fish_timers {
        inc_fish(&mut fish_map, t, 1);
    }

    // simulate fish
    for _ in 1..=ticks {
        fish_map = simulate(fish_map);
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
    println!("[test] number of fish after 80 days: {}", d6_part1("inputs/d6_test"));
    println!("number of fish after 80 days: {}", d6_part1("inputs/d6"));

    println!("[test] number of fish after 256 days: {}", d6_part2("inputs/d6_test"));
    println!("number of fish after 256 days: {}", d6_part2("inputs/d6"));
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
