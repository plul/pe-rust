//! This is a solution to [Project Euler Problem 27](https://projecteuler.net/problem=27).

use common::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::convert::TryFrom;
use std::fmt::Display;

pub fn solve() -> impl Display {
    let mut sieve = SieveOfEratosthenes::new();

    (-999_isize..=999)
        .flat_map(|a| (-1_000_isize..=1_000).map(move |b| (a, b)))
        .max_by_key(|(a, b)| consecutive_primes(*a, *b, &mut sieve))
        .map(|(a, b)| a * b)
        .unwrap()
}

fn consecutive_primes(a: isize, b: isize, sieve: &mut SieveOfEratosthenes) -> usize {
    (0..)
        .map(|n| n * n + a * n + b)
        .filter_map(|x| {
            // Map to usize, while discarding negative numbers that cannot be mapped.
            // Negative numbers are not prime anyway.
            usize::try_from(x).ok()
        }).take_while(|&x| sieve.check_if_prime(x))
        .count()
}

#[test]
fn example() {
    let mut sieve = SieveOfEratosthenes::new();

    assert_eq!(consecutive_primes(1, 41, &mut sieve), 40);
    assert_eq!(consecutive_primes(-79, 1601, &mut sieve), 80);
}
