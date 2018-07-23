use std::collections::HashMap;

fn main() {
    // Map squares of natural numbers to their square root
    let mut squares = HashMap::new();
    for c in 1..1000 {
        squares.insert(c * c, c);
    }

    // Nested iterator, producing pairs of a and b such that a < b < 1000
    let a_b_pairs = (1u64..1000).flat_map(|a| (a..1000).map(move |b| (a, b)));

    // Search for a pair of a and b such that a^2 + b^2 matches a square of
    // a natural number c, and a + b + c = 1000.
    let result = a_b_pairs
        .filter_map(|(a, b)| squares.get(&(a * a + b * b)).map(|c| (a, b, *c)))
        .filter(|(a, b, c)| a + b + c == 1000)
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();

    println!("{}", result);
}
