//! This is a solution to [Project Euler Problem 41](https://projecteuler.net/problem=41).
//!
//! With an upper bound of 987_654_321 this runs in about a minute.
//! However, some clever analysis allows the bound to be set much lower:
//! The sum of the digits of that number is congruent to 9, which means it is divisible by 3, and therefore not prime.
//! The same can be said for any permuration of the number, and for 87_654_321 and its permutation.
//! Therefore the upper bound is 7_654_321.

use common::pandigital::Pandigital;
use common::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::fmt::Display;

pub fn solve() -> impl Display {
    let mut sieve = SieveOfEratosthenes::new();

    let primes: Vec<usize> = sieve.iter().take_while(|&p| p <= 7_654_321).collect();

    primes
        .into_iter()
        .rev()
        .find(|&p| p.is_zeroless_exact_n_pandigital_in_radix(10))
        .unwrap()
}
