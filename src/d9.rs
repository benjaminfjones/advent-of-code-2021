/// AoC 2021 -- Day 9
/// https://adventofcode.com/2021/day/9
use std::collections::HashSet;

use crate::{grid::Grid, util};

type Height = u32;
const MAX_HEIGHT: u32 = 9;
type Risk = u32;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    row: usize,
    col: usize,
    value: Height,
}

type Area = HashSet<Pos>;

/// Represents a (partial) basin, rooted at a fixed low point
#[derive(Debug)]
pub struct Basin {
    root: Pos, // unique low point of the basic, serves as the basin's ID
    area: Area,
}

/// Collect all cardinal grid neighbors with height s.t. `min_height <= height <= max_height`
fn collect_neigbors(
    grid: &Grid<Height>,
    row: usize,
    col: usize,
    min_height: Height,
    max_height: Height,
) -> Area {
    let mut nblocs = Vec::new();
    if row > 0 {
        nblocs.push((row - 1, col));
    }
    if col > 0 {
        nblocs.push((row, col - 1));
    }
    nblocs.push((row + 1, col));
    nblocs.push((row, col + 1));
    nblocs
        .into_iter()
        .map(|(r, c)| {
            grid.get(r, c).map(|&v| Pos {
                row: r,
                col: c,
                value: v,
            })
        })
        .flatten()
        .filter(|p| p.value >= min_height && p.value <= max_height)
        .collect()
}

pub fn windows(grid: &Grid<Height>) -> Vec<Basin> {
    let mut result: Vec<Basin> = Vec::new();
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let nbs: Area = collect_neigbors(grid, row, col, 0, MAX_HEIGHT);
            result.push(Basin {
                root: Pos {
                    row,
                    col,
                    value: *grid.get(row, col).unwrap(),
                },
                area: nbs,
            });
        }
    }
    result
}

pub fn parse_input(input_file: &str) -> Grid<Height> {
    let content = util::read_to_string(input_file).unwrap();
    let rows: Vec<Vec<Height>> = content
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from_rows(rows).expect("failed to build grid")
}

fn find_low_points(grid: &Grid<Height>) -> Area {
    let windows = windows(grid);
    windows
        .into_iter()
        .filter(|w| w.area.iter().all(|n| n.value > w.root.value))
        .map(|w| w.root)
        .collect()
}

/// returns the sum of low point risk scores
pub fn d9_part1(grid: &Grid<Height>) -> Risk {
    find_low_points(grid).iter().map(|p| 1 + p.value).sum()
}

/// returns the combined size of the top 3 largest basins
pub fn d9_part2(grid: &Grid<Height>) -> usize {
    let low_points = find_low_points(grid);
    let mut basins: Vec<(Basin, Area)> = low_points
        .into_iter()
        .map(|p| {
            (
                Basin {
                    root: p.clone(),
                    area: vec![p.clone()].into_iter().collect(),
                },
                // for the initial frontier, collect only neighbors whose height is strictly greater
                // than p's
                collect_neigbors(grid, p.row, p.col, p.value + 1, MAX_HEIGHT - 1),
            )
        })
        .collect();
    loop {
        let mut changed: bool = false;
        let mut new_basins = Vec::new();
        for (basin, frontier) in basins {
            // update basin area w/ current frontier
            let new_area = basin.area.union(&frontier).cloned().collect();
            // grow the frontier
            let mut new_frontier: Area = frontier
                .iter()
                .flat_map(|p| collect_neigbors(grid, p.row, p.col, p.value + 1, MAX_HEIGHT - 1))
                .collect();
            // filter the new frontier to only include positions outside the new area
            new_frontier = new_frontier.difference(&new_area).cloned().collect();
            if !new_frontier.is_empty() {
                changed = true;
            }
            new_basins.push((
                Basin {
                    root: basin.root,
                    area: new_area,
                },
                new_frontier,
            ));
        }
        basins = new_basins;
        if !changed {
            break;
        }
    }
    let mut final_basin_sizes = basins
        .iter()
        .map(|(b, _)| b.area.len())
        .collect::<Vec<usize>>();
    final_basin_sizes.sort_unstable();
    final_basin_sizes = final_basin_sizes.into_iter().rev().collect();
    assert!(final_basin_sizes.len() >= 3);
    final_basin_sizes[0] * final_basin_sizes[1] * final_basin_sizes[2]
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

    #[test]
    fn test_d9_part2_test() {
        let test_grid = parse_input("inputs/d9_test");
        assert_eq!(d9_part2(&test_grid), 1134);
    }

    #[test]
    fn test_d9_part2() {
        let grid = parse_input("inputs/d9");
        assert_eq!(d9_part2(&grid), 964712);
    }
}
