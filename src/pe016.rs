//! This is a solution to [Project Euler Problem 16](https://projecteuler.net/problem=16).

use num::bigint::{BigUint, ToBigUint};
use num::traits::checked_pow;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(1000)
}

fn problem(exp: usize) -> u64 {
    let two: BigUint = 2.to_biguint().unwrap();

    checked_pow(two, exp)
        .unwrap()
        .to_radix_le(10)
        .into_iter()
        .map(u64::from)
        .sum()
}