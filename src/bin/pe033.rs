//! This is a solution to [Project Euler Problem 33](https://projecteuler.net/problem=33).

extern crate num;
extern crate project_euler;

use num::rational::Ratio;
use project_euler::digit_iterator::ToDigits;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let mut targets = Vec::<Ratio<usize>>::new();

    for numer in 10..=99 {
        for denom in numer + 1..=99 {
            if numer % 10 == 0 && denom % 10 == 0 {
                // Trivial case
                continue;
            }

            let numer_digits: Vec<u8> = numer.to_digits();
            let denom_digits: Vec<u8> = denom.to_digits();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 100);
    }
}
