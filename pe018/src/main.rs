//! Progressively work through the triangle, row by row.
//! Compute rows of the maximum value achievable, given that an optimal path is chosen in previous rows,
//! then progress to the next row.
//! Finally, gather the max value in the final row.

use std::cmp::max;
use std::fs::File;
use std::io::Read;
use std::iter;

fn read_triangle(filename: &str) -> Vec<Vec<usize>> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| str::parse::<usize>(s).expect(&format!("Failed to parse as usize: {}", s)))
                .collect()
        })
        .collect()
}

fn main() {
    let data = read_triangle("triangle.txt");

    // The scores for a single row, starting at the first row with only a single number.
    let mut scores: Vec<usize> = vec![data[0][0]];

    for data_row in data.iter().skip(1) {
        // The numbers at the edge, having only a single parent.
        let first: usize = { *scores.first().unwrap() };
        let last: usize = { *scores.last().unwrap() };

        // The numbers in the middle - the max of the two parents.
        let body: Vec<usize> = {
            let left = &scores[1..];
            let right = &scores[..scores.len()];
            left.into_iter()
                .zip(right)
                .map(|(l, r)| *max(l, r))
                .collect()
        };

        // Instantiate the next row underneath, by having each element in the next row inherit
        // from the greater of its one or two parents.
        let expansion: Vec<usize> = iter::once(first)
            .chain(body)
            .chain(iter::once(last))
            .collect();

        // Add numbers from the data triangle.
        scores = expansion
            .into_iter()
            .zip(data_row)
            .map(|(score, data_elem)| score + data_elem)
            .collect();
    }

    let result = scores.iter().max().unwrap();

    println!("{}", result);
}
