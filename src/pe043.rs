//! This is a solution to [Project Euler Problem 43](https://projecteuler.net/problem=43).

use common::util::integer_from_digits;
use std::fmt::Display;

pub fn solve() -> impl Display {
    search(&Vec::<u8>::new())
}

/// Recursive search
fn search(digits: &Vec<u8>) -> usize {
    // Return zero if the condition is not fulfilled:
    if digits.len() >= 4 {
        let divisor = match digits.len() {
            4 => 2,
            5 => 3,
            6 => 5,
            7 => 7,
            8 => 11,
            9 => 13,
            10 => 17,
            _ => panic!("Unexpected length of digit Vec"),
        };

        // Concatenate the last three digits into an integer.
        let tail: usize = {
            let last_three: Vec<u8> = digits.iter().rev().take(3).cloned().collect();
            integer_from_digits(last_three.iter().rev())
        };

        if tail % divisor != 0 {
            return 0;
        }
    }

    // If the pandigital fulfills all requirements, return it.
    if digits.len() == 10 {
        return integer_from_digits(digits.iter());
    }

    // Concatenate new unique digits and recurse
    (0..=9)
        .filter(|d| !digits.contains(d))
        .map(|d| {
            let mut new_vec = digits.to_vec();
            new_vec.push(d);
            search(&new_vec)
        })
        .sum()
}
