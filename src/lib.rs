pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;

pub mod util {
    use std::fs::File;
    use std::io::{self, BufRead, Error, ErrorKind, Read};
    use std::num::ParseIntError;

    /// Return an iterator over lines in a txt file.
    ///
    /// Example:
    /// ```
    /// # use aoc_2021::util::read_lines;
    /// for line in read_lines("inputs/d1").unwrap() {
    ///     if let Ok(ln) = line {
    ///          println!("{}", ln);
    ///     }
    /// }
    /// ```
    pub fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
        let in_file = File::open(path)?;
        Ok(io::BufReader::new(in_file).lines())
    }

    /// Read entire file path into a String
    pub fn read_to_string(path: &str) -> io::Result<String> {
        let mut in_file = File::open(path)?;
        let mut result = String::new();
        in_file.read_to_string(&mut result)?;
        Ok(result)
    }

    pub fn read_ints(path: &str) -> io::Result<Vec<i64>> {
        let content = read_to_string(path)?;
        let ints_res: Result<Vec<_>, ParseIntError> = content
            .trim()
            .split('\n')
            .map(|s| s.parse::<i64>())
            .collect();
        match ints_res {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::Other, "i64 parse error")),
        }
    }
}

mod grid {
    use std::fmt;

    use itertools::iproduct;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Grid<T> {
        pub rows: usize,
        pub cols: usize,
        content: Vec<T>,
    }

    impl<T> Grid<T> {
        pub fn new(rows: usize, cols: usize, fill: T) -> Self
        where
            T: Clone,
        {
            let mut content = Vec::new();
            content.resize(rows * cols, fill);
            Grid {
                rows,
                cols,
                content,
            }
        }

        pub fn from_vec(rows: usize, cols: usize, content: Vec<T>) -> Result<Self, &'static str> {
            if content.len() == rows * cols {
                Ok(Grid {
                    rows,
                    cols,
                    content,
                })
            } else {
                Err("rows * cols vs. content length mismatch")
            }
        }

        pub fn from_rows(content: Vec<Vec<T>>) -> Result<Self, &'static str> {
            if content.is_empty() {
                return Ok(Grid {
                    rows: 0,
                    cols: 0,
                    content: Vec::new(),
                });
            }
            let rows = content.len();
            let cols = content[0].len();
            Ok(Grid {
                rows,
                cols,
                content: content.into_iter().flatten().collect(),
            })
        }

        /// Get element from the grid at (row, col)
        pub fn get(&self, row: usize, col: usize) -> Option<&T> {
            if row < self.rows && col < self.cols {
                Some(&self.content[row * self.cols + col])
            } else {
                None
            }
        }

        /// Get element from the grid at (row, col)
        /// variant that accepts positive and negative row/col
        pub fn geti32(&self, row: i32, col: i32) -> Option<&T> {
            if 0 <= row && row < (self.rows as i32) && 0 <= col && col < (self.cols as i32) {
                Some(&self.content[(row as usize) * self.cols + (col as usize)])
            } else {
                None
            }
        }

        /// Set element on the grid at (row, col)
        pub fn set(&mut self, row: usize, col: usize, value: T) {
            if row < self.rows && col < self.cols {
                self.content[row * self.cols + col] = value;
            } else {
                panic!(
                    "({}, {}) out of bounds for grid size {}x{}",
                    row, col, self.rows, self.cols
                );
            }
        }

        /// Set element on the grid at (row, col)
        /// variant that accepts positive and negative row/col
        pub fn seti32(&mut self, row: i32, col: i32, value: T) {
            if 0 <= row && row < (self.rows as i32) && 0 <= col && col < (self.cols as i32) {
                self.content[(row as usize) * self.cols + (col as usize)] = value;
            } else {
                panic!(
                    "({}, {}) out of bounds for grid size {}x{}",
                    row, col, self.rows, self.cols
                );
            }
        }

        pub fn iter(&self) -> impl Iterator<Item=&T> {
            self.content.iter()
        }

        /// Iterate over position tuples (row, col) in the grid
        pub fn iter_pos(&self) -> impl Iterator<Item=(usize, usize)> {
            iproduct!(0..self.rows, 0..self.cols)
        }

        /// Iterate over tuples (row, col, value) in the grid
        pub fn iter_pos_val(&self) -> impl Iterator<Item=(usize, usize, &T)> {
            self.iter_pos().map(|(r, c)| (r, c, self.get(r, c).unwrap()))
        }

        pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
            self.content.iter_mut()
        }

        pub fn fmt_with_conversion<F>(&self, converter: F) -> String
            where F: Fn(&T) -> String,
        {
            let mut result = String::new();
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let value = self.get(y, x).unwrap();
                    result += &format!("{}", converter(value))
                }
                result += &"\n".to_string();
            }
            result
        }
    }

    // render grids on the terminal, for fun and laughs
    // TODO: don't assume each T renders at the same width
    impl<T: std::fmt::Display> fmt::Display for Grid<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut result = String::new();
            for y in 0..self.rows {
                for x in 0..self.cols {
                    let value = self.get(y, x).unwrap();
                    result += &format!("{}", value)
                }
                result += &"\n".to_string();
            }
            writeln!(f, "{}", result)
        }
    }

    #[cfg(test)]
    mod test_util {
        use super::*;

        #[test]
        fn test_grid_api() {
            let mut grid = Grid::new(5, 5, false);
            assert_eq!(grid.get(0, 0), Some(&false));
            assert_eq!(grid.get(0, 501), None);
            assert_eq!(grid.get(501, 0), None);
            grid.set(1, 1, true);  // does not panic
            assert_eq!(grid.get(1, 1), Some(&true));
        }

        #[test]
        #[should_panic(expected = "(100, 100) out of bounds for grid size 5x5")]
        fn test_grid_set_panic() {
            let mut grid = Grid::new(5, 5, false);
            grid.set(100, 100, true);
        }

        #[test]
        fn test_from_rows() {
            let content = vec![vec![0, 1, 0], vec![2, 3, 4]];
            let grid = Grid::from_rows(content).unwrap();
            assert_eq!(grid.get(0, 0), Some(&0));
        }

        #[test]
        fn test_display_grid() {
            let content = vec![vec![0, 1, 0], vec![2, 3, 4]];
            println!("{}", Grid::from_rows(content).unwrap());
        }
    }
}
