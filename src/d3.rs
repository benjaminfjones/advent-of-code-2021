/// AoC 2021 -- Day 3
/// https://adventofcode.com/2021/day/3
use std::collections::HashSet;

use crate::util;

pub fn d3_part1(input_bvs: &[Vec<bool>]) -> usize {
    let nbits = input_bvs[0].len();
    // gamma_rate has the most common bit at each index
    let gamma_rate: Vec<bool> = (0..nbits)
        .map(|i| mcb(&input_bvs.iter().collect::<Vec<&Vec<bool>>>(), i))
        .collect();
    // eps_rate is the bit-wise negation of gamma_rate
    let eps_rate: Vec<bool> = gamma_rate.iter().map(|b| !*b).collect();
    bv_to_int(&gamma_rate) * bv_to_int(&eps_rate)
}

pub fn d3_part2(input_bvs: &[Vec<bool>]) -> usize {
    let oxy_rate = find_bv_by_selector(input_bvs, |bvs, i| mcb(bvs, i));
    let co2_rate = find_bv_by_selector(input_bvs, |bvs, i| !mcb(bvs, i));
    bv_to_int(oxy_rate) * bv_to_int(co2_rate)
}

/// Alternate implementation of d3_part2, for comparison
pub fn d3_part2_alt2(input_bvs: &[Vec<bool>]) -> usize {
    let oxy_rate = find_bv_by_selector_alt2(input_bvs, true);
    let co2_rate = find_bv_by_selector_alt2(input_bvs, false);
    bv_to_int(oxy_rate) * bv_to_int(co2_rate)
}

/// parse an input file consisting of one bitstring per line into a vector of bitvectors
pub fn input_to_bitvectors(input_file: &str) -> Vec<Vec<bool>> {
    let content = util::read_to_string(input_file).unwrap();
    let bitstrs: Vec<&str> = content.trim().split('\n').collect();
    bitstrs
        .iter()
        .map(|&s| s.chars().map(|c| c == '1').collect::<Vec<bool>>())
        .collect()
}

/// Return the first bitvector after repreatedly filtering using the given column
/// selector; avoids copying inner bitvectors
pub fn find_bv_by_selector<F>(bvs: &[Vec<bool>], selector: F) -> &Vec<bool>
where
    F: Fn(&[&Vec<bool>], usize) -> bool,
{
    let mut current_bvs: Vec<&Vec<bool>> = bvs.iter().collect();
    let nbits = bvs[0].len();
    // row selector, initially all rows are selected
    // refs to bitvectors are copied on each iteration
    for i in 0..nbits {
        let b = selector(&current_bvs, i);
        current_bvs = current_bvs.into_iter().filter(|&v| v[i] == b).collect();
        if current_bvs.len() == 1 {
            break;
        }
    }
    current_bvs[0]
}

/// Alternate implementation of find_bv_by_selector, for comparison
/// This version avoids copying bitvector refs on every iteration, instead removing from a hash
/// set. It also avoids consuming a closure. It is ~100x slower :sob:
pub fn find_bv_by_selector_alt2(bvs: &[Vec<bool>], most_common: bool) -> &Vec<bool> {
    let mut current_bvs: HashSet<&Vec<bool>> = bvs.iter().collect();
    let nbits = bvs[0].len();
    // in this version, bitvector refs are not copied on every iteration
    for i in 0..nbits {
        let b = mcb_alt2(&current_bvs, i) == most_common;
        for v in bvs.iter() {
            if current_bvs.contains(v) && v[i] != b {
                current_bvs.remove(v);
            }
        }
        if current_bvs.len() == 1 {
            break;
        }
    }
    current_bvs.into_iter().next().unwrap()
}

/// Convert bitvector to unsigned int
pub fn bv_to_int(bv: &[bool]) -> usize {
    bv.iter()
        .fold(0, |acc, &x| (acc << 1) + (if x { 1 } else { 0 }))
}

/// Most common bit at given column index
pub fn mcb(bvs: &[&Vec<bool>], index: usize) -> bool {
    2 * bvs.iter().filter(|&&v| v[index]).count() >= bvs.len()
}

/// Most common bit, hashset version
pub fn mcb_alt2(bvs: &HashSet<&Vec<bool>>, index: usize) -> bool {
    2 * bvs.iter().filter(|&&v| v[index]).count() >= bvs.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mcb() {
        let input_bvs = input_to_bitvectors("inputs/d3_test");
        assert_eq!(mcb(&input_bvs.iter().collect::<Vec<&Vec<bool>>>(), 0), true);
        assert_eq!(mcb(&input_bvs.iter().collect::<Vec<&Vec<bool>>>(), 1), false);
    }

    #[test]
    fn test_d3_part1_test() {
        let input_bvs = input_to_bitvectors("inputs/d3_test");
        assert_eq!(d3_part1(&input_bvs), 198);
    }

    #[test]
    fn test_d3_part1() {
        let input_bvs = input_to_bitvectors("inputs/d3");
        assert_eq!(d3_part1(&input_bvs), 2743844);
    }

    #[test]
    fn test_d3_part2_test() {
        let input_bvs = input_to_bitvectors("inputs/d3_test");
        assert_eq!(d3_part2(&input_bvs), 230);
    }

    #[test]
    fn test_d3_part2() {
        let input_bvs = input_to_bitvectors("inputs/d3");
        assert_eq!(d3_part2(&input_bvs), 6677951);
    }

    #[test]
    fn test_d3_part2_alt2() {
        let input_bvs = input_to_bitvectors("inputs/d3");
        assert_eq!(d3_part2_alt2(&input_bvs), 6677951);
    }
}
