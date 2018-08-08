//! This is a solution to [Project Euler Problem 35](https://projecteuler.net/problem=35).

extern crate project_euler;

use project_euler::digit_iterator::ToDigits;
use project_euler::rotations::Rotations;
use project_euler::sieve_of_eratosthenes::SieveOfEratosthenes;
use project_euler::util::integer_from_digit_slice;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1_000_000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> u64 {
    // Find all primes below `n`
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
            let rot_as_integer = integer_from_digit_slice(&rot);
            if !sieve.check_if_prime(rot_as_integer) {
                continue 'outer;
            }
        }

        result += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(problem(100), 13);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(1_000_000), 55);
    }
}
