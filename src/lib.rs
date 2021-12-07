pub mod d7;

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
        let ints_res: Result<Vec<_>, ParseIntError> = content.trim().split('\n').map(|s| s.parse::<i64>()).collect();
        match ints_res {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(ErrorKind::Other, "i64 parse error"))
        }
    }
}
