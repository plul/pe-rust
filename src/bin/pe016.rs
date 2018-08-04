extern crate num;

use num::bigint::{BigUint, ToBigUint};
use num::traits::checked_pow;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(exp: usize) -> u64 {
    let two: BigUint = 2.to_biguint().unwrap();

    checked_pow(two, exp)
        .unwrap()
        .to_radix_le(10)
        .into_iter()
        .map(u64::from)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(1000), 1366);
    }
}
