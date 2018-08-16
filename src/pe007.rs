//! This is a solution to [Project Euler Problem 7](https://projecteuler.net/problem=7).

use common::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::fmt::Display;

pub fn solve() -> impl Display  {
    problem(10000)
}

fn problem(n: usize) -> usize {
    SieveOfEratosthenes::new().iter().nth(n).unwrap()
}