extern crate shared;

use shared::digit_iterator::DigitIterator;
use std::ops::Range;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(Range {
        start: 100,
        end: 1000,
    });
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(range: Range<usize>) -> usize {
    // Produce an iterator for the numbers under consideration
    let end = range.end;
    let products = range.flat_map(|x| (x..end).map(move |y| x * y));

    // Grab the largest palindrome.
    products.filter(|&z| is_palindrome(z)).max().unwrap()
}

/// Iterate from left to right and right to left at the same time, comparing digits.
fn is_palindrome(n: usize) -> bool {
    let from_right = DigitIterator::new(n);
    let from_left = DigitIterator::new(n)
        .collect::<Vec<usize>>()
        .into_iter()
        .rev();

    from_right.zip(from_left).all(|(a, b)| a == b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        let range = Range {
            start: 100,
            end: 1000,
        };
        assert_eq!(problem(range), 906609);
    }
}
