use std::cmp::max;

struct DigitIterator {
    n: u64,
}

impl DigitIterator {
    fn new(n: u64) -> DigitIterator {
        DigitIterator { n: n }
    }
}

// Iteration is from least significant to most significant digit
impl Iterator for DigitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }

        // Extract least significant digit
        let digit = self.n % 10;

        // Discard least significant digit
        self.n /= 10;

        Some(digit)
    }
}

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