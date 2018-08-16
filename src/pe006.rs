//! This is a solution to [Project Euler Problem 6](https://projecteuler.net/problem=6).

use std::fmt::Display;

pub fn solve() -> impl Display {
    problem(100)
}

fn problem(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}

fn sum_of_squares(n: usize) -> usize {
    (1..=n).map(|x| x * x).sum()
}

fn square_of_sum(n: usize) -> usize {
    (1..=n).sum::<usize>().pow(2)
}