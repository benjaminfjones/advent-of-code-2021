/// AoC 2021 -- Day 11
/// https://adventofcode.com/2021/day/11
use std::collections::HashSet;

use crate::{grid::Grid, util};

type Energy = u32;
type OctoGrid = Grid<Energy>;
const MAX_ENERGY: u32 = 9;

pub fn parse_input(input_file: &str) -> OctoGrid {
    let content = util::read_to_string(input_file).unwrap();
    parse_input_from_string(&content)
}

pub fn parse_input_from_string(content: &str) -> OctoGrid {
    let rows: Vec<Vec<Energy>> = content
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from_rows(rows).expect("failed to build grid")
}

/// Add one energy to all octopods, do not flash
fn inc_energy(grid: &mut OctoGrid) {
    for energy in grid.iter_mut() {
        *energy += 1;
    }
}

const ADJACENCIES: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Flash the octopod at (row, col): adds 1 energy to all adjecent (incl. diagonals)
/// octopods and resets center to 0.
fn flash(grid: &mut OctoGrid, row: usize, col: usize) {
    // increment adjacent octopods
    for (dr, dc) in ADJACENCIES {
        let rr = (row as i32) + dr;
        let cc = (col as i32) + dc;
        if let Some(&v) = grid.geti32(rr, cc) {
            grid.seti32(rr, cc, v + 1);
        }
    }
}

/// Find octopods whose energy level is beyond max.
///
/// Ignore octopods at positions in the `ignores` list.
fn find_limit_break_octopus(grid: &OctoGrid, ignores: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    grid.iter_pos_val()
        .filter(|(r, c, &v)|
            !ignores.contains(&(*r, *c)) && v > MAX_ENERGY)
        .map(|(r, c, _)| (r, c))
        .next()
}

/// Simulate `nsteps` of the OctoGrid.
///
/// Return the final grid and the number of flashes that occured.
/// Return the first syncronization step number, or 0 if there was no sync
pub fn simulate(grid: &OctoGrid, nsteps: usize, verbose: bool) -> (OctoGrid, usize, usize) {
    let mut grid = grid.clone();
    let mut nflashes: usize = 0;
    let mut first_sync: usize = 0;
    for step in 0..nsteps {
        if verbose {
            print!("step: {}\n--------\n{}", step, grid);
        }

        // 1. increment octopod energies
        inc_energy(&mut grid);

        // 2. simulate flashes
        let mut flashers: HashSet<(usize, usize)> = HashSet::new();
        while let Some((r, c)) = find_limit_break_octopus(&grid, &flashers) {
            flash(&mut grid, r, c);
            flashers.insert((r, c));
            nflashes += 1;
        }

        // 3. reset the flashers
        if first_sync == 0 && flashers.len() == grid.rows * grid.cols {
            first_sync = step + 1;
        }
        for (r, c) in flashers {
            grid.set(r, c, 0);
        }
    }
    if verbose {
        print!("FINAL\n--------\n{}", grid);
    }
    (grid.clone(), nflashes, first_sync)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let test_grid = parse_input("inputs/d11_test");
        assert_eq!(test_grid.rows, 10);
        assert_eq!(test_grid.cols, 10);

        let grid = parse_input("inputs/d11");
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 10);
    }

    #[test]
    fn test_inc_energy() {
        let content = vec![vec![0, 1, 0], vec![2, 3, 4]];
        let mut grid: OctoGrid = Grid::from_rows(content).unwrap();
        assert_eq!(grid.iter().map(|&v| v).sum::<u32>(), 10);

        inc_energy(&mut grid);
        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(0, 1), Some(&2));
        assert_eq!(grid.iter().map(|&v| v).sum::<u32>(), 16);
    }

    #[test]
    fn test_flash() {
        let content = vec![vec![9, 1, 0, 1], vec![2, 3, 4, 0]];
        let mut grid: OctoGrid = Grid::from_rows(content).unwrap();
        flash(&mut grid, 0, 0);
        assert_eq!(grid.get(0, 0), Some(&9));  // flash doesn't change to center
        assert_eq!(grid.get(0, 1), Some(&2));
        assert_eq!(grid.get(0, 2), Some(&0));
        assert_eq!(grid.get(1, 0), Some(&3));
        assert_eq!(grid.get(1, 1), Some(&4));
        assert_eq!(grid.get(1, 2), Some(&4));
    }

    #[test]
    fn test_find_limit_break() {
        let content = vec![vec![0, 1, 0, 1], vec![2, 9, 4, 0]];
        let mut grid: OctoGrid = Grid::from_rows(content).unwrap();
        assert!(find_limit_break_octopus(&grid, &HashSet::new()).is_none());

        inc_energy(&mut grid);
        let limit_octo = find_limit_break_octopus(&grid, &HashSet::new()).unwrap();
        assert_eq!(limit_octo, (1, 1));
        flash(&mut grid, limit_octo.0, limit_octo.1);
        assert_eq!(grid.get(limit_octo.0, limit_octo.1), Some(&10));
    }

    #[test]
    fn test_simulate() {
        let grid = parse_input_from_string("
            0101
            1090
            0101
        ");
        let (grid_1, nf1, _fs) = simulate(&grid, 1, true);
        assert_eq!(nf1, 1);
        assert_eq!(grid_1.get(0, 0), Some(&1));  // not part of the single flash
        assert_eq!(grid_1.get(0, 2), Some(&2));  // adjacent to the single flash
        assert_eq!(grid_1.get(0, 3), Some(&3));  // adjacent to the single flash
        assert_eq!(grid_1.get(1, 2), Some(&0));  // center of the single flash

        let (grid_2, nf2, _fs) = simulate(&grid, 2, true);
        assert_eq!(nf2, 1);
        assert_eq!(grid_2.get(0, 0), Some(&2));  // not part of the single flash
        assert_eq!(grid_2.get(0, 2), Some(&3));  // adjacent to the single flash
        assert_eq!(grid_2.get(0, 3), Some(&4));  // adjacent to the single flash
        assert_eq!(grid_2.get(1, 2), Some(&1));  // center of the single flash
    }

    #[test]
    fn test_simulate_small_example() {
        let grid = parse_input_from_string("
            11111
            19991
            19191
            19991
            11111
        ");
        let grid_1 = parse_input_from_string("
            34543
            40004
            50005
            40004
            34543
        ");
        let grid_2 = parse_input_from_string("
            45654
            51115
            61116
            51115
            45654
        ");
        assert_eq!(grid, simulate(&grid, 0, false).0);
        assert_eq!(grid_1, simulate(&grid, 1, false).0);
        assert_eq!(grid_2, simulate(&grid, 2, false).0);
    }

    #[test]
    fn test_d11_part1_test() {
        let grid = parse_input("inputs/d11_test");
        let (_final_grid_10, nf, _fs) = simulate(&grid, 10, false);
        assert_eq!(nf, 204);
        let (_final_grid_100, nf, _fs) = simulate(&grid, 100, false);
        assert_eq!(nf, 1656);
    }

    #[test]
    fn test_d11_part1() {
        let grid = parse_input("inputs/d11");
        let (_final_grid_100, nf, _fs) = simulate(&grid, 100, false);
        assert_eq!(nf, 1793);
    }

    #[test]
    fn test_d11_part2_test() {
        let grid = parse_input("inputs/d11_test");
        let (_final_grid_200, _nf, fs) = simulate(&grid, 200, false);
        assert_eq!(fs, 195);
    }

    #[test]
    fn test_d11_part2() {
        let grid = parse_input("inputs/d11");
        let (_final_grid_250, _nf, fs) = simulate(&grid, 250, false);
        assert_eq!(fs, 247);
    }
}
