//! This is solved by realizing that the ways to get to the middle of the grid corresponds to the binomial numbers.
//! The ways to get from the middle to the other corner correspond to the square of the binomial numbers.
//! Then it is just a matter of summing them together.

use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(20);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: u64) -> u64 {
    (0..=n).map(|k| n_choose_k(n, k)).map(|x| x * x).sum()
}

/// Compute the factorial.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Produces the binomial numbers.
fn n_choose_k(n: u64, k: u64) -> u64 {
    factorial(n) / factorial(n - k) / factorial(k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(20), 137846528820);
    }
}
