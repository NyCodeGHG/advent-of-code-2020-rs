use core::panic;
use std::fs;

#[derive(PartialEq)]
pub enum CellState {
    EMPTY,
    TREE,
}

impl CellState {
    /// # Panics
    /// If the input is not a valid cell state
    pub fn from_char(char: &char) -> CellState {
        match char {
            '.' => CellState::EMPTY,
            '#' => CellState::TREE,
            _ => panic!("please die"),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            CellState::EMPTY => '.',
            CellState::TREE => '#',
        }
    }
}

pub fn find_trees(map: &[Vec<CellState>], right: usize, down: usize) -> u32 {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    let x_length = map.first().unwrap().len();
    let y_length = map.len();
    loop {
        x += right;
        x %= x_length;
        y += down;
        if y >= y_length {
            break;
        }
        let cell = map.get(y).unwrap().get(x).unwrap();
        if cell == &CellState::TREE {
            trees += 1;
        }
    }
    trees
}

pub fn read_input() -> Vec<Vec<CellState>> {
    parse_map(&fs::read_to_string("inputs/day03.txt").expect("Unable to read day 3 input"))
}

fn parse_map(text: &str) -> Vec<Vec<CellState>> {
    text.lines()
        .map(|line| line.chars())
        .map(|chars| chars.map(|char| CellState::from_char(&char)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_map() -> Vec<Vec<CellState>> {
        parse_map(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        )
    }

    #[test]
    fn test_first_solution() {
        assert_eq!(find_trees(&test_map(), 3, 1), 7);
    }

    #[test]
    fn test_second_solution() {
        let map = &&test_map();
        let result: u32 = vec![
            find_trees(map, 1, 1),
            find_trees(map, 3, 1),
            find_trees(map, 5, 1),
            find_trees(map, 7, 1),
            find_trees(map, 1, 2),
        ].iter().product();
        assert_eq!(result, 336);
    }
}
