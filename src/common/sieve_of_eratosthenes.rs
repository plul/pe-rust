use bit_vec::BitVec;

pub struct SieveOfEratosthenes {
    head: usize,
    primes: Vec<usize>,
}

impl SieveOfEratosthenes {
    /// Produce a new sieve
    pub fn new() -> SieveOfEratosthenes {
        SieveOfEratosthenes {
            head: 3,
            primes: vec![2],
        }
    }

    pub fn iter(&mut self) -> SieveOfEratosthenesIterator {
        SieveOfEratosthenesIterator::new(self)
    }

    /// Determine if a number is prime
    pub fn check_if_prime(&mut self, n: usize) -> bool {
        while self.largest_prime() < n {
            // need to find more primes
            self.sieve();
        }

        self.primes.binary_search(&n).is_ok()
    }

    /// Return the largest prime found so far.
    pub fn largest_prime(&self) -> usize {
        *self.primes.last().unwrap()
    }

    pub fn get_nth(&mut self, n: usize) -> usize {
        while self.primes.len() <= n {
            // need to find more primes
            self.sieve();
        }
        self.primes[n]
    }

    /// Search for new primes.
    ///
    /// Return the number of newly discovered primes.
    pub fn sieve(&mut self) -> usize {
        let primes_before_search = self.primes.len();

        // Use one megabyte of RAM for the search.
        let n = 1024 * 1024;
        let mut bv = BitVec::from_elem(n, true);

        // Index `i` in the `BitVec` represents the number `i + offset`.
        let offset = self.head;

        let mark_multiples_as_non_prime = |bv: &mut BitVec, head: usize, prime: usize| {
            let align = Self::align(head, prime);
            let non_primes = (0..)
                // Map to the prime and its multiples, aligning with head:
                .map(|x| x * prime + align)
                // Map to indices of the `BitVec`:
                .map(|x| x - offset)
                // Limit the iterator to numbers that can be represented by the current `BitVec`:
                .take_while(|&x| x < n);

            for non_prime_index in non_primes {
                bv.set(non_prime_index, false);
            }
        };

        // Mark multiples of the already discovered primes as non-prime.
        //
        // This maintains the invariant of the sieve; all multiples of primes prior to `head` are marked as non-prime.
        for prime in &self.primes {
            mark_multiples_as_non_prime(&mut bv, self.head, *prime);
        }

        // Sieve through the `BitVec` to extract new primes.
        for _ in 0..n {
            if bv.get(self.head - offset) == Some(true) {
                // head is at a new prime number
                let prime = self.head;
                mark_multiples_as_non_prime(&mut bv, self.head, prime);
                self.primes.push(prime);
            }

            self.head += 1;
        }

        self.primes.len() - primes_before_search
    }

    /// Return a sort of offset that "aligns" a prime with `head`.
    ///
    /// The offset is the first number greater or equal to `head` divisible by `prime`.
    /// Exception: If `head` and `prime` are equal, return the next number.
    ///
    fn align(head: usize, prime: usize) -> usize {
        if head == prime {
            return head + prime;
        }
        if head % prime == 0 {
            return head;
        }
        head / prime * prime + prime
    }
}

impl Default for SieveOfEratosthenes {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SieveOfEratosthenesIterator<'a> {
    sieve: &'a mut SieveOfEratosthenes,
    idx: usize,
}

impl<'a> SieveOfEratosthenesIterator<'a> {
    fn new(sieve: &'a mut SieveOfEratosthenes) -> SieveOfEratosthenesIterator<'a> {
        SieveOfEratosthenesIterator { sieve, idx: 0 }
    }
}

impl<'a> Iterator for SieveOfEratosthenesIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.sieve.get_nth(self.idx);
        self.idx += 1;
        Some(prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align() {
        assert_eq!(SieveOfEratosthenes::align(200, 13), 208);

        assert_eq!(SieveOfEratosthenes::align(208, 13), 208);
        assert_eq!(SieveOfEratosthenes::align(12, 3), 12);
        assert_eq!(SieveOfEratosthenes::align(6, 3), 6);

        assert_eq!(SieveOfEratosthenes::align(13, 13), 26);
        assert_eq!(SieveOfEratosthenes::align(3, 3), 6);
    }

    #[test]
    fn get_nth() {
        let mut sieve = SieveOfEratosthenes::new();
        assert_eq!(sieve.get_nth(0), 2);
        assert_eq!(sieve.get_nth(1), 3);
        assert_eq!(sieve.get_nth(2), 5);
        assert_eq!(sieve.get_nth(3), 7);
        assert_eq!(sieve.get_nth(4), 11);
        assert_eq!(sieve.get_nth(5), 13);
        assert_eq!(sieve.get_nth(6), 17);
        assert_eq!(sieve.get_nth(7), 19);
        assert_eq!(sieve.get_nth(8), 23);
        assert_eq!(sieve.get_nth(9), 29);
    }

    #[test]
    fn iter_basic() {
        let mut sieve = SieveOfEratosthenes::new();
        let mut primes = sieve.iter();
        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
        assert_eq!(primes.next(), Some(23));
        assert_eq!(primes.next(), Some(29));
    }

    #[test]
    fn check_if_prime() {
        let mut sieve = SieveOfEratosthenes::new();
        assert!(sieve.check_if_prime(13));
        assert!(sieve.check_if_prime(23));
        assert!(sieve.check_if_prime(79));
        assert!(sieve.check_if_prime(104743));
    }

    #[test]
    fn pe007() {
        let mut sieve = SieveOfEratosthenes::new();
        let mut primes = sieve.iter();
        assert_eq!(primes.nth(10000), Some(104743));
    }
}