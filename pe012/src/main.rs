extern crate shared;

use shared::prime_factorization::PrimeFactorization;

fn main() {
    let triangle_numbers = (1..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    });

    let result = triangle_numbers
        .map(PrimeFactorization::factor)
        .find(|pf| {
            let numof_factors: usize = pf.prime_factors.iter().map(|f| f.exponent + 1).product();
            numof_factors > 500
        })
        .unwrap()
        .product();

    println!("{}", result);
}
