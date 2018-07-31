extern crate num;
extern crate shared;

use num::bigint::BigUint;
use shared::fibonacci::FibonacciIterator;
use std::ops::Range;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

/// If BigUint supported logarithms, the number of digits in base 10 could be computed
/// by way of the logarithm to base 10.
/// Keep an eye on https://github.com/rust-num/num-bigint/issues/57
///
/// Since the number of digits in base 2 is available, the number of digits in base 10 can
/// be computed approximately, by utilizing the fact that Log[2,n]/Log[10,n] is constant
/// and equal to Log[2]/Log[10] in base e.
fn estimate_number_of_digits_in_base_10(n: &BigUint) -> Range<usize> {
    // Log[2]/Log[10] to base e.
    let ratio: f64 = 0.30102999566398114;

    let f = n.bits() as f64 * ratio;

    Range {
        start: f.floor() as usize,
        end: f.ceil() as usize,
    }
}

/// Returns the index of the first Fibonacci number to contain at least `target_digits` digits.
fn problem(target_digits: usize) -> usize {
    let fib = FibonacciIterator::<BigUint>::new();

    let (idx, _) = fib
        .enumerate()
        .find(|(_, n)| {
            let range = estimate_number_of_digits_in_base_10(n);
            if range.end >= target_digits {
                // Count exact number of digits
                return n.to_radix_le(10).len() >= target_digits;
            }
            false
        })
        .unwrap();

    idx + 1
}
