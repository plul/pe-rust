extern crate shared;

use shared::SieveOfEratosthenes;

fn main() {
    let mut sieve = SieveOfEratosthenes::new();

    let result = sieve.iter().nth(10000).unwrap();
    println!("{}", result);
}
