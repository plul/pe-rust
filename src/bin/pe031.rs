//! This is a solution to [Project Euler Problem 31](https://projecteuler.net/problem=31).

use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let target = 200;
    let result = problem(&coins, target);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
        let target = 200;
        assert_eq!(problem(&coins, target), 73_682);
    }
}
