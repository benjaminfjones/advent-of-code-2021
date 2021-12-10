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

        /// Set element on the grid at (row, col)
        pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), ()> {
            if row < self.rows && col < self.cols {
                self.content[row * self.cols + col] = value;
                Ok(())
            } else {
                Err(())
            }
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

            assert!(grid.set(1, 1, true).is_ok());
            assert_eq!(grid.get(1, 1), Some(&true));
        }

        #[test]
        fn test_from_rows() {
            let content = vec![vec![0, 1, 0], vec![2, 3, 4]];
            let grid = Grid::from_rows(content).unwrap();
            assert_eq!(grid.get(0, 0), Some(&0));
        }
    }
}
