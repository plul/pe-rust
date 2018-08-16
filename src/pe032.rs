//! This is a solution to [Project Euler Problem 32](https://projecteuler.net/problem=32).

use std::collections::BTreeSet;
use common::permutations::Permutations;
use std::fmt::Display;

pub fn solve() -> impl Display {
    let digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutations = Permutations::new(digits);

    let mut discoveries = BTreeSet::new();

    for p in permutations {
            // Playing around with the numbers for a bit reveals that the pattern must be either:
            // a * bbb == cccc
            // or
            // aa * bb == cccc
            let (lhs, rhs) = p.split_at(5);
            for j in 1..=2 {
                let (multiplicant, multiplier) = lhs.split_at(j);

                let multiplicant = integer_from_digits(multiplicant);
                let multiplier = integer_from_digits(multiplier);
                let rhs = integer_from_digits(rhs);

                if multiplicant * multiplier == rhs {
                    discoveries.insert(rhs);
                }
        }
    }

    discoveries.iter().sum::<usize>()
}

fn integer_from_digits(digits: &[u8]) -> usize {
    let mut n = 0;
    for d in digits {
        n *= 10;
        n += usize::from(*d);
    }
    n
}
