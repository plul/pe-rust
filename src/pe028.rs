//! This is a solution to [Project Euler Problem 28](https://projecteuler.net/problem=28).

use std::fmt::Display;
use std::iter;

pub fn solve() -> impl Display {
    problem(1001)
}

fn problem(grid_size: usize) -> usize {
    (1..=grid_size / 2)
        .flat_map(|x| iter::repeat(x * 2).take(4))
        .scan(1, |state, x| {
            *state += x;
            Some(*state)
        }).sum::<usize>()
        + 1
}

#[test]
fn example() {
    assert_eq!(problem(5), 101);
}
