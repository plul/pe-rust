extern crate num_bigint;

use num_bigint::BigUint;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(100);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

/// Computes the factorial.
fn factorial(n: usize) -> BigUint {
    (1..=n).product()
}

fn problem(n: usize) -> u64 {
    let f: BigUint = factorial(n);
    f.to_radix_le(10).into_iter().map(|x| x as u64).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(100), 648);
    }
}
