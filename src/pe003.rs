//! This is a solution to [Project Euler Problem 3](https://projecteuler.net/problem=3).

use std::cmp::max;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(600_851_475_143)
}

fn problem(x: u64) -> u64 {
    largest_prime_factor(x).unwrap()
}

fn first_factor(n: u64) -> u64 {
    (2..n / 2).find(|&x| n % x == 0).unwrap_or(n)
}

fn largest_prime_factor(mut n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }

    let mut lpf = 1u64;

    while n > 1 {
        let ff = first_factor(n);
        lpf = max(lpf, ff);
        n /= ff;
    }

    Some(lpf)
}