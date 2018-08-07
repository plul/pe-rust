//! This is a solution to [Project Euler Problem 29](https://projecteuler.net/problem=29).

extern crate num;

use num::bigint::{BigUint, ToBigUint};
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(100);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    (2..=n)
        .flat_map(|a| (2..=n).map(move |b| (a, b)))
        .map(|(a, b)| {
            let base = a.to_biguint().unwrap();
            let exp = b;
            num::pow::pow(base, exp)
        }).collect::<HashSet<BigUint>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(problem(5), 15);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(100), 0);
    }
}
