/// AoC 2021 -- Day 1
/// https://adventofcode.com/2021/day/1

use crate::util;

pub fn main() {
    let d1_part1_soln = d1_part1("inputs/d1");
    println!("Day 1, part 1 solution: {}", d1_part1_soln);

    let d1_part2_soln = d1_part2("inputs/d1");
    println!("Day 1, part 2 solution: {}", d1_part2_soln);
}

pub fn d1_part1(input_file: &str) -> usize {
    let es = util::read_ints(input_file).unwrap();
    let len = es.len();

    let preds = &es[0..len];
    let succs = &es[1..];
    preds
        .iter()
        .zip(succs)
        .filter(|(&p, &s)| s > p)
        .collect::<Vec<_>>()
        .len()
}

pub fn d1_part2(input_file: &str) -> usize {
    let es = util::read_ints(input_file).unwrap();
    let windows: Vec<&[i64]> = es.windows(3).collect();
    let len = windows.len();
    let preds = &windows[0..len];
    let succs = &windows[1..];
    preds
        .iter()
        .zip(succs)
        .filter(|(&pw, &sw)| sw[2] > pw[0])
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d1_part1() {
        assert_eq!(d1_part1("inputs/d1_test"), 7);
        assert_eq!(d1_part1("inputs/d1"), 1451);
    }

    #[test]
    fn test_d1_part2() {
        assert_eq!(d1_part2("inputs/d1_test"), 5);
        assert_eq!(d1_part2("inputs/d1"), 1395);
    }
}
