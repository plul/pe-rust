use bit_vec::BitVec;

/// Let index i in the BitVec correspond to the number i.
pub struct SieveOfEratosthenes(BitVec);

impl SieveOfEratosthenes {
    /// Produce a new sieve
    pub fn new() -> SieveOfEratosthenes {
        let mut sieve = SieveOfEratosthenes(BitVec::from_elem(8, true));
        sieve.0.set(0, false); // 0 is not a prime
        sieve.0.set(1, false); // 1 is not a prime
        sieve.sieve();
        sieve
    }

    // Iterate through primes, dynamically growing the sieve as necessary.
    pub fn iter(&mut self) -> SieveOfEratosthenesIterator {
        SieveOfEratosthenesIterator::new(self)
    }

    /// Double the size of the sieve
    fn grow(&mut self) {
        {
            let bv = &mut self.0;
            let n = bv.len();
            bv.grow(n, true);
        }
        self.sieve();
    }

    /// Walk through the bitvector, classifying numbers as non-prime
    fn sieve(&mut self) {
        let bv = &mut self.0;
        let len = bv.len();
        for i in 2..len {
            if bv.get(i) == Some(true) {
                // i is a prime.
                for j in (i..).map(|x| x * i).take_while(|&x| x < len) {
                    // Mark all multiples of i as non-prime
                    bv.set(j, false);
                }
            }
        }
    }
}

impl Default for SieveOfEratosthenes {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator with a mutable reference to the sieve.
/// It will grow the sieve while iterating, as needed.
pub struct SieveOfEratosthenesIterator<'a> {
    sieve: &'a mut SieveOfEratosthenes,
    index: usize,
}

impl<'a> SieveOfEratosthenesIterator<'a> {
    fn new(sieve: &'a mut SieveOfEratosthenes) -> SieveOfEratosthenesIterator<'a> {
        SieveOfEratosthenesIterator {
            sieve,
            index: 0,
        }
    }
}

impl<'a> Iterator for SieveOfEratosthenesIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;

            if self.index >= self.sieve.0.len() {
                self.sieve.grow();
            }

            if self.sieve.0.get(self.index) == Some(true) {
                return Some(self.index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut sieve = SieveOfEratosthenes::new();
        assert_eq!(sieve.iter().nth(0), Some(2));
        assert_eq!(sieve.iter().nth(1), Some(3));
        assert_eq!(sieve.iter().nth(2), Some(5));
    }

    #[test]
    fn pe007() {
        let mut sieve = SieveOfEratosthenes::new();
        assert_eq!(sieve.iter().nth(10000), Some(104743));
    }
}
