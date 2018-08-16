//! This is a solution to [Project Euler Problem 5](https://projecteuler.net/problem=5).

use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(20)
}

fn problem(upper: usize) -> usize {
    (1..)
        .map(|x| x * upper)
        .find(|&x| is_divisible_up_to(x, upper))
        .unwrap()
}

fn is_divisible_up_to(n: usize, upper: usize) -> bool {
    (1..upper).rev().all(|x| n % x == 0)
}