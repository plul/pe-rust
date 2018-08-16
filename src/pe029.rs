//! This is a solution to [Project Euler Problem 29](https://projecteuler.net/problem=29).

use num::bigint::{BigUint, ToBigUint};
use num::pow;
use std::collections::HashSet;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(100)
}

fn problem(n: usize) -> usize {
    (2..=n)
        .flat_map(|a| (2..=n).map(move |b| (a, b)))
        .map(|(a, b)| {
            let base = a.to_biguint().unwrap();
            let exp = b;
            pow::pow(base, exp)
        }).collect::<HashSet<BigUint>>()
        .len()
}

#[test]
fn example() {
    assert_eq!(problem(5), 15);
}
