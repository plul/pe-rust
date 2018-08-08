//! This is a solution to [Project Euler Problem 30](https://projecteuler.net/problem=30).

#![feature(no_panic_pow)]

extern crate project_euler;

use project_euler::digit_iterator::ToDigits;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(5);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(p: u32) -> u32 {
    (2_u32..=largest_number_that_has_to_be_checked(p))
        .filter(|&x| {
            x == x
                .to_digits()
                .into_iter()
                .map(|digit| u32::from(digit))
                .map(|digit| digit.checked_pow(p).unwrap())
                .sum()
        }).sum()
}

/// Find the largest number that has to be checked.
///
/// All numbers above are guaranteed to be greater than the sum of their digits raised to the power `p`.
fn largest_number_that_has_to_be_checked(p: u32) -> u32 {
    let num_of_digits = (1_u32..)
        .find(|&num_of_digits| {
            let max_digit_sum = 9_u32.pow(p) * num_of_digits;
            let max_integer_out_of_digits = 10_u32.pow(num_of_digits) - 1;

            max_digit_sum < max_integer_out_of_digits
        }).unwrap();

    9_u32.pow(p) * num_of_digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boundary() {
        assert_eq!(largest_number_that_has_to_be_checked(4), 32_805);
        assert_eq!(largest_number_that_has_to_be_checked(5), 354_294);
    }

    #[test]
    fn example() {
        assert_eq!(problem(4), 19316);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(5), 443_839);
    }
}
