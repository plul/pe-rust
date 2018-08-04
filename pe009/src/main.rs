use std::collections::HashMap;

use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    // Map squares of natural numbers to their square root
    let mut squares = HashMap::new();
    for c in 1..1000 {
        squares.insert(c * c, c);
    }

    // Nested iterator, producing pairs of a and b such that a < b < 1000
    let a_b_pairs = (1..1000).flat_map(|a| (a..1000).map(move |b| (a, b)));

    // Search for a pair of a and b such that a^2 + b^2 matches a square of
    // a natural number c, and a + b + c = 1000.
    a_b_pairs
        .filter_map(|(a, b)| squares.get(&(a * a + b * b)).map(|c| (a, b, *c)))
        .find(|(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 31875000);
    }
}
