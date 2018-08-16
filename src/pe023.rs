//! This is a solution to [Project Euler Problem 23](https://projecteuler.net/problem=23).

use std::fmt::Display;

pub fn solve() -> impl Display {
    let mut abundant_numbers: Vec<usize> = Vec::new();

    // table[n] indicates whether n is expressible as the sum of two abundant numbers.
    let mut table = vec![false; 28123];

    let mut result = 0;

    for n in 1..28123 {
        if !table[n] {
            // n cannot be expressed as the sum of two abundant numbers.
            result += n;
        }

        if divisors(n).sum::<usize>() > n {
            // n is abundant.
            abundant_numbers.push(n);

            // Match n with any previously found abundant number, and mark the sums as expressible by the sum of two abundant numbers.
            for an in &abundant_numbers {
                let s = an + n;
                if s >= table.len() {
                    break;
                }
                table[s] = true;
            }
        }
    }

    result
}

fn divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n / 2).filter(move |x| n % x == 0)
}

#[test]
fn divisors_of_28() {
    assert_eq!(divisors(28).sum::<usize>(), 28);
}
