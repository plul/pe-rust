//! This is a solution to [Project Euler Problem 36](https://projecteuler.net/problem=36).

use num::bigint::BigUint;
use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(1_000_000)
}

fn problem(n: u64) -> u64 {
    (1..n)
        .filter(|&x| is_palindromic_in_base(x, 2) && is_palindromic_in_base(x, 10))
        .sum()
}

fn is_palindromic_in_base(x: u64, base: u8) -> bool {
    let b = BigUint::from(x);
    let digits = b.to_radix_le(u32::from(base));
    let digits_reversed = get_reversed_vec(&digits);
    digits == digits_reversed
}

fn get_reversed_vec<T>(v: &Vec<T>) -> Vec<T> where T: Clone {
    let mut clone = (*v).clone();
    clone.reverse();
    clone
}
