//! This is a solution to [Project Euler Problem 33](https://projecteuler.net/problem=33).

use num::rational::Ratio;
use crate::common::radix::Radix;
use std::fmt::Display;

pub fn solve() -> impl Display  {
    let mut targets = Vec::<Ratio<usize>>::new();

    for numer in 10..=99 {
        for denom in numer + 1..=99 {
            if numer % 10 == 0 && denom % 10 == 0 {
                // Trivial case
                continue;
            }

            let numer_digits: Vec<u8> = numer.to_radix_le(10);
            let denom_digits: Vec<u8> = denom.to_radix_le(10);

            let mut incorrectly_simplified = Vec::new();

            if numer_digits[0] == denom_digits[1] && denom_digits[0] != 0 {
                incorrectly_simplified.push((numer_digits[1], denom_digits[0]));
            }
            if numer_digits[1] == denom_digits[0] && denom_digits[1] != 0 {
                incorrectly_simplified.push((numer_digits[0], denom_digits[1]));
            }

            for (numer_digit, denom_digit) in incorrectly_simplified {
                let rational_incorrectly_simplified = Ratio::<usize>::new(usize::from(numer_digit), usize::from(denom_digit));
                if rational_incorrectly_simplified == Ratio::<usize>::new(numer, denom) {
                    targets.push(rational_incorrectly_simplified.reduced());
                }
            }
        }
    }

    *targets.iter().product::<Ratio<usize>>().reduced().denom()
}