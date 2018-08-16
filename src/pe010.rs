//! This is a solution to [Project Euler Problem 10](https://projecteuler.net/problem=10).

use common::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(2_000_000)
}

fn problem(x: usize) -> u64 {
    SieveOfEratosthenes::new()
        .iter()
        .take_while(|&n| n < x)
        .map(|n| n as u64)
        .sum::<u64>()
}
