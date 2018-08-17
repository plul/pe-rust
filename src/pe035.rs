//! This is a solution to [Project Euler Problem 35](https://projecteuler.net/problem=35).

use common::digit_iterator::ToDigits;
use common::rotations::Rotations;
use common::sieve_of_eratosthenes::SieveOfEratosthenes;
use common::util::integer_from_digit_slice;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(1_000_000)
}

fn problem(n: usize) -> u64 {
    let mut result = 0;

    let mut sieve = SieveOfEratosthenes::new();

    'outer: for i in 0.. {
        let prime = sieve.get_nth(i);

        if prime > n {
            break;
        }

        let mut digits: Vec<u8> = prime.to_digits();
        digits.reverse();

        'inner: for rot in Rotations::new(digits) {
            let rot_as_integer = integer_from_digit_slice(rot.iter());
            if !sieve.check_if_prime(rot_as_integer) {
                continue 'outer;
            }
        }

        result += 1;
    }

    result
}

#[test]
fn example() {
    assert_eq!(problem(100), 13);
}
