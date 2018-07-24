//! This is solved by realizing that the ways to get to the middle of the grid corresponds to the binomial numbers.
//! The ways to get from the middle to the other corner correspond to the square of the binomial numbers.
//! Then it is just a matter of summing them together.

/// Computes the factorial.
fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// Produces the binomial numbers.
fn n_choose_k(n: usize, k: usize) -> usize {
    factorial(n) / factorial(n - k) / factorial(k)
}

fn main() {
    let n = 20;
    let k_iter = 0..=n;

    let result: usize = k_iter.map(|k| n_choose_k(n, k)).map(|x| x * x).sum();

    println!("{}", result);
}
