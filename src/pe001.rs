//! This is a solution to [Project Euler Problem 1](https://projecteuler.net/problem=1).

use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(1000)
}

fn problem(x: u64) -> u64 {
    (1..x).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

#[test]
fn test() {
    assert_eq!(problem(10), 23);
}
