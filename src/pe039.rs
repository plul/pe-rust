//! This is a solution to [Project Euler Problem 39](https://projecteuler.net/problem=39).

use std::fmt::Display;

pub fn solve() -> impl Display {
    (1..=1000)
        .max_by_key(|&p| problem(p))
        .unwrap()
}

fn problem(p: usize) -> usize {
    (1..=p/2)
        .flat_map(|a| (a..=p/2).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, p - a - b))
        .filter(|(a, b, c)| b < c && a*a + b*b == c*c)
        .count()
}

#[test]
fn example() {
    assert_eq!(problem(120).to_string(), "3");
}
