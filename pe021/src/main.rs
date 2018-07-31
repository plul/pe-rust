//! It may be that it is faster to compute the prime factorization, and
//! then derive the divisors from that.

use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(10000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    (1..n)
        .filter_map(|n| {
            let s = sum_of_divisors(n);
            if s < n && n == sum_of_divisors(s) {
                return Some(s + n);
            }
            None
        })
        .sum()
}

fn sum_of_divisors(n: usize) -> usize {
    (1..=n / 2).filter(|&x| n % x == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem(10000), 31626);
    }
}
