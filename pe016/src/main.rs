extern crate num_bigint;
extern crate num_traits;
extern crate shared;

use num_bigint::{BigUint, ToBigUint};
use num_traits::pow;
use shared::digit_iterator::DigitIterator;

fn main() {
    let x: BigUint = 2u8.to_biguint().unwrap();
    let x: BigUint = pow(x, 1000);

    let result: BigUint = DigitIterator::new(x).sum();

    println!("{}", result);
}
