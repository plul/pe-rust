//! This is a solution to [Project Euler Problem 40](https://projecteuler.net/problem=40).

use crate::common::radix::Radix;
use std::fmt::Display;

pub fn solve() -> impl Display {
    vec![1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]
        .into_iter()
        .map(|n| problem(n - 1))
        .map(|digit| u64::from(digit))
        .product::<u64>()
}

/// Find the n'th digit in the fractional part (zero-indexed)
fn problem(n: usize) -> u8 {
    (1..)
        .flat_map(|x| x.to_radix_be(10).into_iter())
        .nth(n)
        .unwrap()
}

#[test]
fn example() {
    assert_eq!(problem(12), 1);
}
