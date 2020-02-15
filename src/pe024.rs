//! This is a solution to [Project Euler Problem 24](https://projecteuler.net/problem=24).
//!
//! This is a kind of search algorithm - it does not generate all possible permutations.
//!
//! There will be an equal number of permutations starting with any given character,
//! So if the input is {0, 1, 2}, then there are equally many permutations arising
//! from that set that have 0, 1 or 2 as the first digit.
//! Furthermore, those are all ordered, so it is fairly simple to figure out which
//! character should come first, and then the same logic can be applied for the next...

use crate::common::permutations::Permutations;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(1_000_000 - 1)
}

fn problem(target: usize) -> String {
    let chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    Permutations::new(chars)
        .get_permutation(target)
        .into_iter()
        .collect()
}