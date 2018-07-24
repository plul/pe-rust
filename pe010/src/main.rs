extern crate shared;

use shared::sieve_of_eratosthenes::SieveOfEratosthenes;

fn main() {
    let mut sieve = SieveOfEratosthenes::new();
    let result: usize = sieve.iter().take_while(|n| *n < 2_000_000).sum();

    println!("{}", result);
}
