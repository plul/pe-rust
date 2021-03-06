//! This is a solution to [Project Euler Problem 37](https://projecteuler.net/problem=37).

use crate::common::radix::Radix;
use crate::common::sieve_of_eratosthenes::SieveOfEratosthenes;
use crate::common::util::integer_from_digits;
use std::collections::VecDeque;
use std::fmt::Display;

pub fn solve() -> impl Display {
    let mut sieve = SieveOfEratosthenes::new();
    let mut sieve_2 = SieveOfEratosthenes::new();

    sieve
        .iter()
        .filter(|&p| p >= 10)
        .filter(|&p| is_truncatable_prime(p, &mut sieve_2))
        .take(11)
        .sum::<usize>()
}

fn is_truncatable_prime(prime: usize, sieve: &mut SieveOfEratosthenes) -> bool {
    let digits_vec: Vec<u8> = prime.to_radix_be(10);

    // Check in one direction
    let mut digits: VecDeque<u8> = digits_vec.iter().cloned().collect();
    digits.pop_front();
    while !digits.is_empty() {
        let n: usize = integer_from_digits(digits.iter());
        if !sieve.check_if_prime(n) {
            return false;
        }
        digits.pop_front();
    }

    // Check in other direction
    let mut digits: VecDeque<u8> = digits_vec.iter().cloned().collect();
    digits.pop_back();
    while !digits.is_empty() {
        let n: usize = integer_from_digits(digits.iter());
        if !sieve.check_if_prime(n) {
            return false;
        }
        digits.pop_back();
    }

    true
}

#[test]
fn example() {
    let mut sieve = SieveOfEratosthenes::new();
    assert!(is_truncatable_prime(3797, &mut sieve));
}
