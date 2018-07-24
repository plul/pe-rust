extern crate shared;

use shared::DigitIterator;

use std::cmp::max;

/// Iterate from left to right and right to left at the same time, comparing digits.
fn is_palindrome(n: u64) -> bool {
    let from_right = DigitIterator::new(n);
    let from_left = DigitIterator::new(n)
        .collect::<Vec<u64>>()
        .into_iter()
        .rev();

    from_right.zip(from_left).all(|(a, b)| a == b)
}

fn main() {
    // Produce an iterator for the numbers under consideration
    let products = (100..1000).flat_map(|x| (x..1000).map(move |y| x * y));

    // Grab the largest palindrome.
    let result = products
        .filter(|z| is_palindrome(*z))
        .fold(0, |acc, n| max(acc, n));

    println!("{}", result);
}