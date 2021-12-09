/// AoC 2021 -- Day 5
/// https://adventofcode.com/2021/day/5
extern crate aoc_2021;

use std::collections::HashMap;
use std::fmt;

use regex::Regex;

use aoc_2021::util;

/// 2d line on an integer grid
#[derive(Debug)]
struct Line {
    start: (i64, i64),
    end: (i64, i64),
}

impl Line {
    // create a new line, normalizing start/end to be lexicographic
    fn new(x0: i64, y0: i64, x1: i64, y1: i64) -> Self {
        if x0 > x1 {
            Line {
                start: (x1, y1),
                end: (x0, y0),
            }
        } else if x0 == x1 && y0 > y1 {
            Line {
                start: (x1, y1),
                end: (x0, y0),
            }
        } else {
            Line {
                start: (x0, y0),
                end: (x1, y1),
            }
        }
    }

    fn is_horiz(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vert(&self) -> bool {
        self.start.0 == self.end.0
    }
}

/// 2d integer grid position
type Pos = (i64, i64);

/// A 2d integer grid containing counts at each coordinate.
///
/// Implemented as a map of position -> #intersecting vents. The upper left corner is (0,0), x
/// increases to the right, y increases downwards
#[derive(Debug)]
struct Grid(HashMap<Pos, usize>);

impl Grid {
    fn new() -> Self {
        Grid(HashMap::new())
    }

    fn get_count(&self, pos: &Pos) -> usize {
        *self.0.get(pos).unwrap_or(&0)
    }

    fn xmax(&self) -> Option<i64> {
        self.0.keys().map(|&p| p.0).max()
    }

    fn ymax(&self) -> Option<i64> {
        self.0.keys().map(|&p| p.1).max()
    }

    fn increment(&mut self, pos: Pos) {
        self.0.insert(pos, self.get_count(&pos) + 1);
    }

    fn add_line(&mut self, line: Line) {
        if line.is_horiz() {
            for x in line.start.0..=line.end.0 {
                self.increment((x, line.start.1));
            }
        } else if line.is_vert() {
            for y in line.start.1..=line.end.1 {
                self.increment((line.start.0, y));
            }
        } else {
            // assume line is diagonal
            let dy = line.end.1 - line.start.1;
            let dx = line.end.0 - line.start.0;
            let slope = dy / dx;
            let mut y = line.start.1;
            for x in line.start.0..=line.end.0 {
                self.increment((x, y));
                y += slope;
            }
        }
    }
}

// render grids on the terminal, just for fun :)
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xmax = self.xmax().unwrap_or(0);
        let ymax = self.ymax().unwrap_or(0);
        let mut result = String::new();
        for y in 0..=ymax {
            for x in 0..=xmax {
                let count = self.get_count(&(x, y));
                let marker = if count > 0 {
                    format!("{}", count)
                } else {
                    ".".to_string()
                };
                result += &marker;
            }
            result += &format!("\n");
        }
        write!(f, "{}\n", result)
    }
}

// TODO: cleanup parsing, eliminate repeated capture code
fn parse_input(file: &str) -> Vec<Line> {
    let content = util::read_to_string(file).unwrap();
    let line_re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let mut result: Vec<Line> = Vec::new();
    for line in content.trim().split('\n') {
        let tr_line = line.trim();
        assert!(line_re.is_match(tr_line));
        let captures = line_re.captures(line).unwrap();
        let x0 = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y0 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let x1 = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let y1 = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        result.push(Line::new(x0, y0, x1, y1));
    }
    result
}

fn d5_part1(file: &str) -> usize {
    let mut grid = Grid::new();
    let vents = parse_input(file);
    for line in vents {
        if line.is_horiz() || line.is_vert() {
            grid.add_line(line);
        }
    }
    grid.0.values().filter(|&&c| c > 1).count()
}

fn d5_part2(file: &str) -> usize {
    let mut grid = Grid::new();
    let vents = parse_input(file);
    for line in vents {
        grid.add_line(line);
    }
    grid.0.values().filter(|&&c| c > 1).count()
}

pub fn main() {
    println!(
        "# dangerous areas (w/o diagonal vents): {}",
        d5_part1("inputs/d5")
    );
    println!("# dangerous areas (all vents): {}", d5_part2("inputs/d5"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d5_part1() {
        assert_eq!(d5_part1("inputs/d5_test"), 5);
        assert_eq!(d5_part1("inputs/d5"), 6189);
    }

    #[test]
    fn test_d5_part2() {
        assert_eq!(d5_part2("inputs/d5_test"), 12);
        assert_eq!(d5_part2("inputs/d5"), 19164);
    }
}
