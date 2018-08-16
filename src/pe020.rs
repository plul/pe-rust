//! This is a solution to [Project Euler Problem 20](https://projecteuler.net/problem=20).

use num::bigint::BigUint;
use std::fmt::Display;

pub fn solve() -> impl Display  {
    problem(100)
}

fn problem(n: usize) -> u64 {
    let f: BigUint = factorial(n);
    f.to_radix_le(10).into_iter().map(u64::from).sum()
}

/// Computes the factorial.
fn factorial(n: usize) -> BigUint {
    (1..=n).product()
}