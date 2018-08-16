//! This is a solution to [Project Euler Problem 31](https://projecteuler.net/problem=31).

use std::fmt::Display;

pub fn solve() -> impl Display {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let target = 200;
    problem(&coins, target)
}

/// Compute the answer recursively.
///
/// The coins must be sorted in descending order.
fn problem(coins: &[usize], target: usize) -> usize {
    if target == 0 {
        return 1;
    }

    coins
        .iter()
        .enumerate()
        .filter(|(_, &coin)| target >= coin)
        .map(|(idx, coin)| problem(&coins[idx..], target - coin))
        .sum()
}
