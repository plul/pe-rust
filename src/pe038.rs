//! This is a solution to [Project Euler Problem 38](https://projecteuler.net/problem=38).

use common::util::integer_from_digits;
use common::radix::Radix;
use common::pandigital::Pandigital;
use std::fmt::Display;

pub fn solve() -> impl Display {
    let search_space = (1..50).chain(100..=333).chain(5000..10000);

    search_space
        .filter_map(|x| -> Option<u32> {
            // Try to achieve a 9 digit number by multiplying and concatenating.
            let mut digits: Vec<u8> = Vec::new();
            for multiplier in 1.. {
                digits.extend((x * multiplier).to_radix_be(10));
                if digits.len() == 9 {
                    return Some(integer_from_digits(digits.iter()));
                }
                if digits.len() > 9 {
                    return None;
                }
            }
            None
        })
        .filter(|x| x.is_zeroless_pandigital_in_radix(10))
        .max()
        .unwrap()
}