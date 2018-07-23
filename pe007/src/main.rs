extern crate bit_vec;

use bit_vec::BitVec;

/// Let index i in the BitVec correspond to the number i.
struct SieveOfEratosthenes(BitVec);

impl SieveOfEratosthenes {
    /// Produce a new sieve
    fn new() -> SieveOfEratosthenes {
        let mut sieve = SieveOfEratosthenes(BitVec::from_elem(8, true));
        sieve.0.set(0, false); // 0 is not a prime
        sieve.0.set(1, false); // 1 is not a prime
        sieve._sieve();
        sieve
    }

    /// Double the size of the sieve
    fn _grow(&mut self) {
        {
            let bv = &mut self.0;
            let n = bv.len();
            bv.grow(n, true);
        }
        self._sieve();
    }

    fn _sieve(&mut self) {
        let bv = &mut self.0;
        let len = bv.len();
        for i in 2..len {
            if bv.get(i) == Some(true) {
                // i is a prime.
                for j in (i..).map(|x| x * i).take_while(|x| x < &len) {
                    // Mark all multiples of i as non-prime
                    bv.set(j, false);
                }
            }
        }
    }

    /// Search for the n'th prime,
    /// Grow the sieve if the n'th prime is not included in the sieve, and try again.
    fn find(&mut self, target: u64) -> u64 {
        let mut primes = 0;
        for (n, value) in (0..).zip(self.0.iter()) {
            if value {
                primes += 1;
            }
            if primes == target {
                return n;
            }
        }
        self._grow();
        self.find(target)
    }
}

fn main() {
    let target = 10001;

    let mut sieve = SieveOfEratosthenes::new();
    let result = sieve.find(target);

    println!("{}", result);
}
