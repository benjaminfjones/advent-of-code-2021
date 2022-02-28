/// AoC 2021 -- Day 13
/// https://adventofcode.com/2021/day/13

use crate::{grid::Grid, util};

type DotGrid = Grid<bool>;

pub fn parse_input(input_file: &str) -> DotGrid {
    let content = util::read_to_string(input_file).unwrap();
    parse_input_from_string(&content)
}

pub fn parse_input_from_string(content: &str) -> DotGrid {
    let lines: Vec<&str> = content
        .trim()
        .split('\n')
        .collect();
    assert!(lines.len() >= 4);  // at least one coord, one blank line, 2 folds
    let coord_lines = &lines[..lines.len()-3];
    let xys: Vec<(usize, usize)> = coord_lines.into_iter()
        .map(|line| {
            let cs: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap()).collect();
            assert!(cs.len() == 2);
            (cs[0], cs[1])
        })
        .collect();
    let nrows = xys.iter().map(|(_x, y)| *y).max().unwrap() + 1;
    let ncols = xys.iter().map(|(x, _y)| *x).max().unwrap() + 1;
    let mut grid: DotGrid = Grid::new(nrows, ncols, false);
    // (x, y) -> (col, row)
    for (col, row) in xys {
        grid.set(row, col, true);
    }
    // TODO: parse and convert last two fold lines
    grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_test_input() {
        let grid = parse_input("inputs/d13_test");
        assert_eq!(grid.get(0, 0), Some(&false));
        println!(
            "{}",
            grid.fmt_with_conversion(|b| if *b {"x".to_string()} else {".".to_string()})
        );
    }
}
