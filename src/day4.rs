use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

type Coord = (i64, i64);
// Encode a particular orientation of xmas
type Encoding = [Coord; 4];


const FORWARD: Encoding = [
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
];
const DOWN: Encoding = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
];
const DIAGONAL_DOWN: Encoding = [
    (0, 0),
    (1, 1),
    (2, 2),
    (3, 3),
];
const DIAGONAL_UP: Encoding = [
    (0, 0),
    (1, -1),
    (2, -2),
    (3, -3),
];


const CASES: [Encoding; 4] = [
    FORWARD,
    DOWN,
    DIAGONAL_UP,
    DIAGONAL_DOWN,
];



struct LetterGrid {
    pub grid: HashMap<Coord, String>
}

impl LetterGrid {
    fn get(&self, x: i64, y: i64) -> &str {
        match self.grid.get(&(x, y)) {
            Some(letter) => letter,
            None => "",
        }
    }

    /// Check four directions for XMAS and SAMX
    fn check_xmases(&self, x: i64, y: i64) -> usize {
        let mut n = 0;
        for case in CASES {
            let mut s = String::new();
            for (dx, dy) in case {
                s += self.get(x + dx, y + dy);
            }
            if s == "XMAS" {
                n += 1
            } else if s == "SAMX" {
                n += 1
            }
        }
        n
    }
}


/// Read input as map of coords -> letters at said coords
fn read_input() -> LetterGrid {
    let file = File::open("inputs/day4.txt").unwrap();

    // Store each cell according to its coords
    let mut grid = HashMap::new();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            grid.insert((x as i64, y as i64), char.to_string());
        }
    }

    LetterGrid {
        grid
    }
}

/// Check every position in the grid for potential xmases
fn find_xmas(letter_grid: &LetterGrid) -> usize {
    let mut xmases = 0;

    for (x, y) in letter_grid.grid.keys() {
        xmases += letter_grid.check_xmases(*x, *y);
    }

    xmases
}

fn main() {
    let input = read_input();
    let result = find_xmas(&input);
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input() -> LetterGrid {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        // Store each cell according to its coords
        let mut grid = HashMap::new();
        for (y, line) in input.split("\n").enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid.insert((x as i64, y as i64), char.to_string());
            }
        }
        LetterGrid {
            grid
        }
    }

    #[test]
    fn simple_case() {
        let grid = make_input();
        assert_eq!(find_xmas(&grid), 18);
    }
}
