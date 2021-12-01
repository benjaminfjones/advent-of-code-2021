pub mod util {
    use std::fs::File;
    use std::io::{self, BufRead, Read};

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
}
