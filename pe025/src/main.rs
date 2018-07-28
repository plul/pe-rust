extern crate num;
extern crate shared;

use num::bigint::BigUint;
use num::{One, Zero};
use shared::digit_iterator::DigitIterator;
use std::mem::swap;
use std::ops::{Add, Range};

struct FibonacciIterator<T: Zero + One + Clone>(T, T)
where
    for<'a> &'a T: Add<Output = T>;

impl<T: Zero + One + Clone> FibonacciIterator<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    fn new() -> FibonacciIterator<T> {
        FibonacciIterator(T::zero(), T::one())
    }
}

impl<T: Zero + One + Clone> Iterator for FibonacciIterator<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = &self.0 + &self.1;
        swap(&mut self.0, &mut self.1);

        Some(self.0.clone())
    }
}

/// If BigUint supported logarithms, the number of digits in base 10 could be computed
/// by way of the logarithm to base 10.
/// However, since the number of digits in base 2 is available, the number of digits in
/// base 10 can be computed approximately.
/// This used the fact that Log[2,n]/Log[10,n] is constant and equal to Log[2]/Log[10].
fn approx_number_of_digits_in_base_10(n: &BigUint) -> Range<u64> {
    // This is the ratio between the natural logaritm of 2 to the natural logarithm of 10
    let ratio: f64 = 0.30102999566398114;

    let f = n.bits() as f64 * ratio;

    Range {
        start: f.floor() as u64,
        end: f.ceil() as u64,
    }
}

fn main() {
    let fib = FibonacciIterator::<BigUint>::new();

    let (idx, _) = fib
        .enumerate()
        .find(|(_, n)| {
            let range = approx_number_of_digits_in_base_10(n);
            if range.end >= 1000 {
                // Count exact number of digits
                return DigitIterator::new(n.clone()).count() >= 1000;
            }
            false
        })
        .unwrap();

    let result = idx + 1;

    println!("{}", result);
}
