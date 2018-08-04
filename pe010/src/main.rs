extern crate shared;

use shared::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(2_000_000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(x: usize) -> u64 {
    SieveOfEratosthenes::new()
        .iter()
        .take_while(|&n| n < x)
        .map(|n| n as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(2_000_000), 142913828922);
    }
}
