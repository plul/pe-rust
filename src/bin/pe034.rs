//! This is a solution to [Project Euler Problem 34](https://projecteuler.net/problem=34).

extern crate project_euler;

use project_euler::digit_iterator::DigitIterator;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> u64 {
    let to = largest_number_that_has_to_be_checked();
    (3_u64..=to)
        .filter(|&x| x == DigitIterator::new(x).map(factorial).sum())
        .sum()
}

fn factorial(n: u64) -> u64 {
    (2..=n).product()
}

/// Find the largest number that has to be checked.
///
/// All numbers above are guaranteed to be greater than the sum of the factorials of their digits.
fn largest_number_that_has_to_be_checked() -> u64 {
    let num_of_digits = (1_u32..)
        .find(|&num_of_digits| {
            let max_sum_of_digit_factorials = factorial(9) * u64::from(num_of_digits);
            let max_integer_out_of_digits = 10_u64.pow(num_of_digits) - 1;

            max_sum_of_digit_factorials < max_integer_out_of_digits
        }).unwrap();

    factorial(9) * u64::from(num_of_digits)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 40730);
    }
}
