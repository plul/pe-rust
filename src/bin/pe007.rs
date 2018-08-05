extern crate project_euler;

use project_euler::sieve_of_eratosthenes::SieveOfEratosthenes;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(10000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    SieveOfEratosthenes::new().nth(n).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(10000), 104743);
    }
}
