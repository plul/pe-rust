//! This is a solution to [Project Euler Problem 25](https://projecteuler.net/problem=25).
//!
//! The 1000th fibonacci number is too great to be represented with a 64 or 128 bit integer.
//! Therefore, I am using BigUint from the num crate.
//! The number of digits in a number represented in base 10 is straight forward to compute as a function of the logarithm of the number to base 10.
//! However, `BigUint` does not yet support such logarithms, although there is an open [issue on GitHub requesting they be implemented](https://github.com/rust-num/num-bigint/issues/57).
//!
//! Workaround:
//! I cannot take the logarithm to base 10, but I do have access to ⌈log_2(n)⌉ by way of the `bits()` method on `BigUint`.
//! I can therefore cheaply produce an estimate for the number of digits in base 10.
//! See the helper function [estimate_number_of_digits_in_base_10](estimate_number_of_digits_in_base_10).
//! When necessary, the BigUint is represented as digits in base 10 by way of the [to_radix_le](num::bigint::BigUint::to_radix_le) method, in order to accurately count the digits.

use crate::common::fibonacci::FibonacciIterator;
use num::bigint::BigUint;
use std::fmt::Display;
use std::ops::Range;

pub fn solve() -> impl Display {
    problem(1000)
}

/// Return the index of the first Fibonacci number to contain at least the given number of digits.
fn problem(target_digits: usize) -> usize {
    let fib = FibonacciIterator::<BigUint>::new();

    let (idx, _) = fib
        .enumerate()
        .find(|(_, n)| {
            let range = estimate_number_of_digits_in_base_10(n);
            if range.end >= target_digits {
                // Count exact number of digits
                return n.to_radix_le(10).len() >= target_digits;
            }
            false
        }).unwrap();

    idx + 1
}

/// Estimate number of digits in the base 10 representation of a number.
///
/// Since the number of digits in base 2 is available, the number of digits in base 10 can be computed approximately, by utilizing the fact that log_2(n) divided by log_10(n) is constant and equal to ln(2) divided by ln(10).
fn estimate_number_of_digits_in_base_10(n: &BigUint) -> Range<usize> {
    // ln(2) / ln(10).
    let ratio: f64 = 0.301_029_995_663_981_14;

    let f = n.bits() as f64 * ratio;

    Range {
        start: f.floor() as usize,
        end: f.ceil() as usize,
    }
}

#[test]
fn three_digits() {
    assert_eq!(problem(3), 12);
}
