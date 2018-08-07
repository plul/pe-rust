//! This is a solution to [Project Euler Problem 28](https://projecteuler.net/problem=28).

use std::time::Instant;
use std::iter;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1001);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(grid_size: usize) -> usize {
    (1..=grid_size / 2)
        .flat_map(|x| iter::repeat(x * 2).take(4))
        .scan(1, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .sum::<usize>() + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(problem(5), 101);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(problem(1001), 669_171_001);
    }
}
