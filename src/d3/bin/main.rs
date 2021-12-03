/// AoC 2021 -- Day 3
/// https://adventofcode.com/2021/day/3

extern crate aoc_2021;

use aoc_2021::util;

pub fn main() {
    println!("d3 part1: {}", d3_part1("inputs/d3"));
    println!("d3 part2: {}", d3_part2("inputs/d3"));
}

pub fn d3_part1(input_file: &str) -> usize {
    let input_bvs = input_to_bitvectors(input_file);
    let nbits = input_bvs[0].len();
    let mut gamma_rate: usize = 0;
    let mut eps_rate: usize = 0;
    for i in 0..nbits {
        gamma_rate = gamma_rate << 1;
        eps_rate = eps_rate << 1;
        if mcb(&input_bvs, i) == 0 {
            eps_rate += 1;
        } else {
            gamma_rate += 1;
        }
    }
    gamma_rate * eps_rate
}

pub fn d3_part2(input_file: &str) -> usize {
    let input_bvs = input_to_bitvectors(input_file);
    let oxy_rate = find_bv_by_selector(&input_bvs, |bvs, i| mcb(bvs, i));
    let co2_rate = find_bv_by_selector(&input_bvs, |bvs, i| 1 - mcb(bvs, i));
    oxy_rate * co2_rate
}

/// parse an input file consisting of one bitstring per line into a vector of bitvectors
pub fn input_to_bitvectors(input_file: &str) -> Vec<Vec<usize>> {
    let content = util::read_to_string(input_file).unwrap();
    let bitstrs: Vec<&str> = content.trim().split('\n').collect();
    bitstrs.iter()
        .map(|&s| s.chars().map(|c| c.to_digit(2).unwrap() as usize).collect::<Vec<usize>>())
        .collect()
}

/// Return the int value of the first bitvector after repreatedly filtering using the given column
/// selector
pub fn find_bv_by_selector<F>(bvs: &Vec<Vec<usize>>, selector: F) -> usize
    where F: Fn(&Vec<Vec<usize>>, usize) -> usize {
    let nbits = bvs[0].len();
    let mut bvs_clone = bvs.clone();
    for i in 0..nbits {
        let b = selector(&bvs_clone, i);
        bvs_clone = bvs_clone.into_iter().filter(|v| v[i] == b).collect();
        if bvs_clone.len() == 1 {
            break;
        }
    }
    bv_to_int(&bvs_clone[0])
}

/// Convert bitvector to unsigned int
pub fn bv_to_int(bv: &Vec<usize>) -> usize {
    bv.iter().fold(0, |acc, &x| (acc << 1) + x)
}

/// Most common bit at given index
pub fn mcb(bvs: &Vec<Vec<usize>>, index: usize) -> usize {
    let col_sum: usize = bvs.iter().map(|v| v[index]).sum();
    if 2 * col_sum >= bvs.len() {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d3_part1() {
        assert_eq!(d3_part1("inputs/d3_test"), 198);
        assert_eq!(d3_part1("inputs/d3"), 2743844);
    }

    #[test]
    fn test_d3_part2() {
        assert_eq!(d3_part2("inputs/d3_test"), 230);
        assert_eq!(d3_part2("inputs/d3"), 6677951);
    }
}
