extern crate num_bigint;
extern crate shared;

use num_bigint::BigUint;
use shared::digit_iterator::DigitIterator;

/// Computes the factorial.
fn factorial(n: usize) -> BigUint {
    (1..=n).product()
}

fn main() {
    let f = factorial(100);
    let di = DigitIterator::new(f);
    let result: BigUint = di.sum();

    println!("{}", result);
}
