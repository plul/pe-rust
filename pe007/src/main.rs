extern crate shared;

use shared::sieve_of_eratosthenes::SieveOfEratosthenes;

fn main() {
    let mut sieve = SieveOfEratosthenes::new();

    let result = sieve.iter().nth(10000).unwrap();
    println!("{}", result);
}
