extern crate project_euler;

use project_euler::prime_factorization::PrimeFactorization;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(500);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    let triangle_numbers = (1..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    });

    let pf: PrimeFactorization = triangle_numbers
        .map(PrimeFactorization::factor)
        .find(|pf| {
            let numof_factors: usize = pf.prime_factors.iter().map(|f| f.exponent + 1).product();
            numof_factors > n
        }).unwrap();

    pf.product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(500), 76576500);
    }
}
