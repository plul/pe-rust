//! This is a solution to [Project Euler Problem 12](https://projecteuler.net/problem=12).

use crate::common::prime_factorization::PrimeFactorization;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(500)
}

fn problem(n: usize) -> usize {
    let triangle_numbers = (1..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    });

    let pf: PrimeFactorization = triangle_numbers
        .map(PrimeFactorization::factor)
        .find(|pf| {
            let numof_factors: usize = pf.prime_factors.iter().map(|f| f.exponent + 1).product();
            numof_factors > n
        }).unwrap();

    pf.product()
}