use bit_vec::BitVec;

pub struct SieveOfEratosthenes {
    head: usize,
    bv: BitVec,
    primes: Vec<usize>,
}

impl SieveOfEratosthenes {
    /// Produce a new sieve
    pub fn new() -> SieveOfEratosthenes {
        let mut bv = BitVec::from_elem(8, true);
        bv.set(0, false); // 0 is not a prime
        bv.set(1, false); // 1 is not a prime
        SieveOfEratosthenes {
            head: 0,
            bv,
            primes: Vec::new(),
        }
    }

    /// Double the size of the sieve
    fn grow(&mut self) {
        {
            let bv = &mut self.bv;
            let n = bv.len();
            bv.grow(n, true);
        }

        for prime in &self.primes {
            Self::mark_multiples_as_non_prime(&mut self.bv, self.head, *prime);
        }
    }

    /// Mark multiples of the prime as non-prime, skipping values before head.
    fn mark_multiples_as_non_prime(bv: &mut BitVec, head: usize, prime: usize) {
        let skip = head - head % prime;
        let len = bv.len();
        for non_prime in (0..).map(|x| skip + x * prime).take_while(|&x| x < len) {
            bv.set(non_prime, false);
        }
    }
}

/// The underlying bitvec will be expanded while iterating, as needed.
impl Iterator for SieveOfEratosthenes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.head += 1;

            if self.head >= self.bv.len() {
                self.grow();
            }

            if self.bv.get(self.head) == Some(true) {
                // head is at a new prime number
                let prime = self.head;
                Self::mark_multiples_as_non_prime(&mut self.bv, self.head, prime);
                self.primes.push(prime);

                return Some(prime);
            }
        }
    }
}

impl Default for SieveOfEratosthenes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut sieve = SieveOfEratosthenes::new();
        assert_eq!(sieve.next(), Some(2));
        assert_eq!(sieve.next(), Some(3));
        assert_eq!(sieve.next(), Some(5));
    }

    #[test]
    fn pe007() {
        let mut sieve = SieveOfEratosthenes::new();
        assert_eq!(sieve.nth(10000), Some(104743));
    }
}
