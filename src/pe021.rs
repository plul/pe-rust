//! This is a solution to [Project Euler Problem 21](https://projecteuler.net/problem=21).
//!
//! It may be that it is faster to compute the prime factorization, and
//! then derive the divisors from that.

use std::fmt::Display;


pub fn solve() -> impl Display  {
    problem(10000)
}

fn problem(n: usize) -> usize {
    (1..n)
        .filter_map(|n| {
            let s = sum_of_divisors(n);
            if s < n && n == sum_of_divisors(s) {
                return Some(s + n);
            }
            None
        }).sum()
}

fn sum_of_divisors(n: usize) -> usize {
    (1..=n / 2).filter(|&x| n % x == 0).sum()
}