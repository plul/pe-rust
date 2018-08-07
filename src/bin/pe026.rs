//! This is a solution to [Project Euler Problem 26](https://projecteuler.net/problem=26).

use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    (2..n)
        .max_by_key(|&x| number_of_recurring_digits(x))
        .unwrap()
}

/// Find the number of recurring digits in a unit fraction.
///
/// This produces digits by long division.
/// When a non-zero numerator repeats, it indicates the start of a recurring cycle.
fn number_of_recurring_digits(denominator: usize) -> usize {
    type Numerator = usize;

    let mut numerator: Numerator = 1;
    let mut v = Vec::<Numerator>::new();

    loop {
        numerator %= denominator;
        numerator *= 10;

        if let Some((idx, _)) = v.iter().enumerate().find(|&(_idx, &num)| num == numerator) {
            return if numerator > 0 { v.len() - idx } else { 0 };
        }

        v.push(numerator);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(number_of_recurring_digits(2), 0);
        assert_eq!(number_of_recurring_digits(3), 1);
        assert_eq!(number_of_recurring_digits(4), 0);
        assert_eq!(number_of_recurring_digits(5), 0);
        assert_eq!(number_of_recurring_digits(6), 1);
        assert_eq!(number_of_recurring_digits(7), 6);
        assert_eq!(number_of_recurring_digits(8), 0);
        assert_eq!(number_of_recurring_digits(9), 1);
        assert_eq!(number_of_recurring_digits(10), 0);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(1000), 983);
    }
}
