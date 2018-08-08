extern crate project_euler;

use project_euler::digit_iterator::ToDigits;
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
    let digits: Vec<u8> = n.to_digits();

    let reversed = {
        let mut clone = digits.clone();
        clone.reverse();
        clone
    };

    digits == reversed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        let range = Range {
            start: 100,
            end: 1000,
        };
        assert_eq!(problem(range), 906609);
    }
}
