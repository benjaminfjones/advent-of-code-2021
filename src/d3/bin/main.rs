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
    // gamma_rate has the most common bit at each index
    let gamma_rate: Vec<bool> = (0..nbits)
        .map(|i| mcb(&input_bvs.iter().collect(), i))
        .collect();
    // eps_rate is the bit-wise negation of gamma_rate
    let eps_rate = gamma_rate.iter().map(|b| !b).collect();
    bv_to_int(&gamma_rate) * bv_to_int(&eps_rate)
}

pub fn d3_part2(input_file: &str) -> usize {
    let input_bvs = input_to_bitvectors(input_file);
    let oxy_rate = find_bv_by_selector(&input_bvs, |bvs, i| mcb(bvs, i));
    let co2_rate = find_bv_by_selector(&input_bvs, |bvs, i| !mcb(bvs, i));
    bv_to_int(&oxy_rate) * bv_to_int(&co2_rate)
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
pub fn find_bv_by_selector<F>(bvs: &Vec<Vec<bool>>, selector: F) -> &Vec<bool>
where
    F: Fn(&Vec<&Vec<bool>>, usize) -> bool,
{
    let nbits = bvs[0].len();
    let mut bv_ptrs: Vec<&Vec<bool>> = bvs.iter().collect();
    for i in 0..nbits {
        let b = selector(&bv_ptrs, i);
        bv_ptrs = bv_ptrs.into_iter().filter(|&v| v[i] == b).collect();
        if bv_ptrs.len() == 1 {
            break;
        }
    }
    bv_ptrs[0]
}

/// Convert bitvector to unsigned int
pub fn bv_to_int(bv: &Vec<bool>) -> usize {
    bv.iter()
        .fold(0, |acc, &x| (acc << 1) + (if x { 1 } else { 0 }))
}

/// Most common bit at given index
pub fn mcb(bvs: &Vec<&Vec<bool>>, index: usize) -> bool {
    2 * bvs.iter().filter(|&v| v[index]).count() >= bvs.len()
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
