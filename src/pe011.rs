//! This is a solution to [Project Euler Problem 11](https://projecteuler.net/problem=11).

use std::cmp::max;
use std::fmt::Display;
use std::str;

const DATA: &[u8] = include_bytes!("pe011-data.txt");

enum Direction {
    South,
    East,
    NorthEast,
    SouthEast,
}

struct Grid {
    data: Vec<Vec<usize>>,
}

impl Grid {
    /// Parse the data file
    fn from(data: &str) -> Grid {
        let data: Vec<Vec<usize>> = data
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| {
                        str::parse::<usize>(s)
                            .unwrap_or_else(|_| panic!("Failed to parse as usize: {}", s))
                    }).collect()
            }).collect();

        Grid { data }
    }

    /// Given a starting point and a direction, produce elements in a given direction until the edge of the grid is reached.
    fn line<'a>(
        &'a self,
        start_row: usize,
        start_col: usize,
        direction: &Direction,
    ) -> Vec<&usize> {
        let mut r = start_row;
        let mut c = start_col;
        let mut v: Vec<&'a usize> = Vec::new();

        while let Some(elem) = self.data.get(r).and_then(|row| row.get(c)) {
            v.push(elem);
            match direction {
                Direction::South => {
                    r += 1;
                }
                Direction::East => {
                    c += 1;
                }
                Direction::NorthEast => {
                    if r == 0 {
                        break;
                    }
                    r -= 1;
                    c += 1;
                }
                Direction::SouthEast => {
                    r += 1;
                    c += 1;
                }
            };
        }

        v
    }
}

pub fn solve() -> impl Display {
    problem(20)
}

fn problem(n: usize) -> usize {
    let data: &str = str::from_utf8(DATA).unwrap();
    let grid = Grid::from(data);

    // East going lines starting from column 0
    let east = (0..n).map(|row_idx| grid.line(row_idx, 0, &Direction::East));

    // South going lines starting from row 0
    let south = (0..n).map(|col_idx| grid.line(0, col_idx, &Direction::South));

    // There is a bit of overlap in the following lines, but that does not matter as we are looking for the
    // maximum, not the sum or product.

    // South east going lines starting at column 0
    let south_east_1 = (0..n).map(|row_idx| grid.line(row_idx, 0, &Direction::SouthEast));

    // South east going lines starting at row 0
    let south_east_2 = (0..n).map(|col_idx| grid.line(0, col_idx, &Direction::SouthEast));

    // North east going lines starting at column 0
    let north_east_1 = (0..n).map(|row_idx| grid.line(row_idx, 0, &Direction::NorthEast));

    // North east going lines starting at the last row
    let north_east_2 = (0..n).map(|col_idx| grid.line(n - 1, col_idx, &Direction::NorthEast));

    // This becomes an iterator of lines going in various directions, (where the lines are of type Vec<&usize>).
    let lines = east
        .chain(south)
        .chain(south_east_1)
        .chain(south_east_2)
        .chain(north_east_1)
        .chain(north_east_2);

    let mut result: usize = 0;
    for line in lines {
        // Cut every line into windows of elements
        let windows = line.windows(4);
        for window in windows {
            // Compute the product
            let p: usize = window.iter().map(|x| *x).product();

            // Store the largest seen product
            result = max(result, p);
        }
    }

    result
}
