/// AoC 2021 -- Day 9
/// https://adventofcode.com/2021/day/9
use crate::{grid::Grid, util};

#[allow(dead_code)]
pub struct Window {
    row: usize, // row, col currently unused
    col: usize,
    value: u32,
    neighbors: Vec<u32>,
}

pub fn windows(grid: &Grid<u32>) -> Vec<Window> {
    let mut result: Vec<Window> = Vec::new();
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let nbs = vec![
                if row > 0 {
                    grid.get(row - 1, col)
                } else {
                    None
                },
                grid.get(row + 1, col),
                if col > 0 {
                    grid.get(row, col - 1)
                } else {
                    None
                },
                grid.get(row, col + 1),
            ]
            .into_iter()
            .flatten()
            .map(|v| *v)
            .collect();
            result.push(Window {
                row,
                col,
                value: *grid.get(row, col).unwrap(),
                neighbors: nbs,
            });
        }
    }
    result
}

pub fn parse_input(input_file: &str) -> Grid<u32> {
    let content = util::read_to_string(input_file).unwrap();
    let rows: Vec<Vec<u32>> = content
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from_rows(rows).expect("failed to build grid")
}

/// returns the sum of low point risk scores
pub fn d9_part1(grid: &Grid<u32>) -> u32 {
    let windows = windows(grid);
    windows
        .into_iter()
        .filter(|w| w.neighbors.iter().all(|&nv| nv > w.value))
        .map(|w| 1 + w.value)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let test_grid = parse_input("inputs/d9_test");
        assert_eq!(test_grid.rows, 5);
        assert_eq!(test_grid.cols, 10);

        let grid = parse_input("inputs/d9");
        assert_eq!(grid.rows, 100);
        assert_eq!(grid.cols, 100);
    }

    #[test]
    fn test_d9_part1_test() {
        let test_grid = parse_input("inputs/d9_test");
        assert_eq!(d9_part1(&test_grid), 15);
    }

    #[test]
    fn test_d9_part1() {
        let grid = parse_input("inputs/d9");
        assert_eq!(d9_part1(&grid), 588);
    }
}
