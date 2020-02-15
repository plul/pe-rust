//! This is a solution to [Project Euler Problem 4](https://projecteuler.net/problem=4).

use crate::common::radix::Radix;
use std::ops::Range;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(Range {
        start: 100,
        end: 1000,
    })
}

fn problem(range: Range<usize>) -> usize {
    // Produce an iterator for the numbers under consideration
    let end = range.end;
    let products = range.flat_map(|x| (x..end).map(move |y| x * y));

    // Grab the largest palindrome.
    products.filter(|&z| is_palindrome(z)).max().unwrap()
}

fn is_palindrome(n: usize) -> bool {
    let digits: Vec<u8> = n.to_radix_le(10);

    let reversed = {
        let mut clone = digits.clone();
        clone.reverse();
        clone
    };

    digits == reversed
}