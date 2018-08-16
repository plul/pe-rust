//! This is a solution to [Project Euler Problem 2](https://projecteuler.net/problem=2).

use common::fibonacci::FibonacciIterator;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(4_000_000)
}

fn problem(x: u64) -> u64 {
    let f = FibonacciIterator::<u64>::new();
    f.take_while(|&n| n < x).filter(|&n| n % 2 == 0).sum()
}