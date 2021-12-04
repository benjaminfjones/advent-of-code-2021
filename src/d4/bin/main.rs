/// AoC 2021 -- Day 4
/// https://adventofcode.com/2021/day/4

extern crate aoc_2021;

use std::collections::HashMap;
use std::collections::HashSet;
use aoc_2021::util;

pub fn main() {
    // Process the munged input
    let mut input_iter = util::read_lines("inputs/d4_munge").unwrap();
    let first_line = input_iter.next().unwrap().unwrap();
    let bingo_calls: Vec<u32> = first_line.split(',').map(|w| w.parse::<u32>().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();
    for mline in input_iter {
        let line = mline.unwrap();
        let vs: Vec<u32> = line.split(' ').map(|w| w.parse::<u32>().unwrap()).collect();
        boards.push(Board::from_u32s(&vs));
    }

    // Play bingo!
    //
    // first printed WIN is the score for the first board to win,
    // last printed WIN is the last board to win
    let mut boards_won: HashSet<usize> = HashSet::new();
    for call in bingo_calls {
        for (i, board) in boards.iter_mut().enumerate() {
            if boards_won.contains(&i) { continue }
            if board.mark_and_win(call) {
                println!("WIN {}", board.sum_unmarked() * call);
                boards_won.insert(i);
            }
        }
    }
}

/// row, col position on a board
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pos(usize, usize);

/// cell represents a board value and mark state
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Cell {
    value: u32,
    mark: bool,
}

impl Cell {
    pub fn new(value: u32, mark: bool) -> Self {
        Cell {value, mark}
    }
}

// a 5x5 board of cells
#[derive(Debug)]
pub struct Board {
    // position -> cell
    cellmap: HashMap<Pos, Cell>,
    // cell value -> position
    posmap: HashMap<u32, Pos>,
}

impl Board {
    pub fn from_u32s(us: &Vec<u32>) -> Self {
        assert!(us.len() == 25);
        let mut cellmap: HashMap<Pos, Cell> = HashMap::new();
        let mut posmap: HashMap<u32, Pos> = HashMap::new();
        for (i, &u) in us.iter().enumerate() {
            let pos = Pos(i / 5, i % 5);
            let cell = Cell::new(u, false);
            cellmap.insert(pos, cell);
            posmap.insert(u, pos);
        }
        Board {cellmap, posmap}
    }

    // check if the (row, col) is marked; assume row, col are in bounds
    fn is_marked(&self, row: usize, col: usize) -> bool {
        self.cellmap.get(&Pos(row, col)).unwrap().mark
    }

    // Mark the number on the board, mutating the board. If the number was present on the board and
    // marking results in a winning board, return `true`, otherwise `false`.
    pub fn mark_and_win(&mut self, num: u32) -> bool {
        if !self.posmap.contains_key(&num) { return false }
        let pos = self.posmap.get(&num).unwrap();
        let newcell = Cell::new(num, true);
        self.cellmap.insert(*pos, newcell);

        // check for row win along row == pos.0
        let rowwin = (0..5).all(|col| self.is_marked(pos.0, col));
        // check for col win along col == pos.1
        let colwin = (0..5).all(|row| self.is_marked(row, pos.1));
        rowwin || colwin
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.cellmap.iter()
            .map(|(_, c)| c)
            .filter(|c| !c.mark)
            .map(|c| c.value)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_mark_board() {
        let grid = vec![
           22, 13, 17, 11,  0,
            8,  2, 23,  4, 24,
           21,  9, 14, 16,  7,
            6, 10,  3, 18,  5,
            1, 12, 20, 15, 19,
        ];
        let grid_sum: u32 = grid.iter().sum();
        let mut board1 = Board::from_u32s(&grid);
        // test a column win, called in row order
        assert!(!board1.mark_and_win(22));
        assert!(!board1.mark_and_win(8));
        assert!(!board1.mark_and_win(21));
        assert!(!board1.mark_and_win(6));
        // WIN
        assert!(board1.mark_and_win(1));
        assert_eq!(board1.sum_unmarked(), grid_sum - 22 - 8 - 21 - 6 - 1);

        // test a row win, in a non-sequential order with extra calls
        let mut board2 = Board::from_u32s(&grid);
        assert!(!board2.mark_and_win(6));
        assert!(!board2.mark_and_win(3));
        assert!(!board2.mark_and_win(5));
        assert!(!board2.mark_and_win(18));
        assert!(!board2.mark_and_win(0));  // not in row
        assert!(!board2.mark_and_win(19));  // not in row
        // WIN
        assert!(board2.mark_and_win(10));
        assert_eq!(board2.sum_unmarked(), grid_sum - 6 - 3 - 5 - 18 - 19 - 10);

        // test a diagonal non-win
        let mut board3 = Board::from_u32s(&grid);
        assert!(!board3.mark_and_win(0));
        assert!(!board3.mark_and_win(4));
        assert!(!board3.mark_and_win(14));
        assert!(!board3.mark_and_win(10));
        assert!(!board3.mark_and_win(1));
    }
}

