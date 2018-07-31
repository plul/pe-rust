use std::cmp::max;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(600851475143);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(x: u64) -> u64 {
    largest_prime_factor(x).unwrap()
}

fn first_factor(n: u64) -> u64 {
    (2..n / 2).find(|&x| n % x == 0).unwrap_or(n)
}

fn largest_prime_factor(mut n: u64) -> Option<u64> {
    if n < 2 {
        return None;
    }

    let mut lpf = 1u64;

    while n > 1 {
        let ff = first_factor(n);
        lpf = max(lpf, ff);
        n /= ff;
    }

    Some(lpf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem(600851475143), 6857);
    }
}
