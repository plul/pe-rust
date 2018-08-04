extern crate project_euler;

use project_euler::fibonacci::FibonacciIterator;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(4_000_000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(x: u64) -> u64 {
    let f = FibonacciIterator::<u64>::new();

    f.take_while(|&n| n < x).filter(|&n| n % 2 == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(4_000_000), 4613732);
    }
}