extern crate bit_vec;
extern crate itertools;
extern crate num_bigint;

use bit_vec::BitVec;
use itertools::Itertools;
use std::iter;
use std::ops::{DivAssign, Rem};

pub struct Factor {
    pub n: usize,
    pub exponent: usize,
}

pub struct PrimeFactorization {
    pub prime_factors: Vec<Factor>,
}

impl PrimeFactorization {
    pub fn factor(mut n: usize) -> PrimeFactorization {
        let mut prime_factors = Vec::<usize>::new();

        let mut d = 2;
        while n > 1 {
            if n % d == 0 {
                n /= d;
                prime_factors.push(d);
            } else {
                d += 1;
            }
        }

        let prime_factors_grouped = prime_factors
            .into_iter()
            .group_by(|x| *x)
            .into_iter()
            .map(|(k, group)| Factor {
                n: k,
                exponent: group.count(),
            })
            .collect();

        PrimeFactorization {
            prime_factors: prime_factors_grouped,
        }
    }

    pub fn product(&self) -> usize {
        self.prime_factors
            .iter()
            .flat_map(|factor| iter::repeat(factor.n).take(factor.exponent))
            .product()
    }
}

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
                for j in (i..).map(|x| x * i).take_while(|x| x < &len) {
                    // Mark all multiples of i as non-prime
                    bv.set(j, false);
                }
            }
        }
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
            sieve: sieve,
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

pub struct DigitIterator<N: From<u8> + Clone + PartialEq + DivAssign + Rem<Output = N>> {
    n: N,
}

impl<N: From<u8> + Clone + PartialEq + DivAssign + Rem<Output = N>> DigitIterator<N> {
    pub fn new(n: N) -> DigitIterator<N> {
        DigitIterator { n: n }
    }
}

// Iteration is from least significant to most significant digit
impl<N: From<u8> + Clone + PartialEq + DivAssign + Rem<Output = N>> Iterator for DigitIterator<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0.into() {
            return None;
        }

        // Extract least significant digit
        let digit = (self.n).clone() % 10.into();

        // Discard least significant digit
        self.n /= 10.into();

        Some(digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn u64_0() {
        let zero = 0u64;
        let mut digits = DigitIterator::new(zero);
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn u64_1() {
        let one = 1u64;
        let mut digits = DigitIterator::new(one);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn u64_10() {
        let ten = 10u64;
        let mut digits = DigitIterator::new(ten);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn u64_12345678() {
        let large_int = 12345678u64;
        let mut digits = DigitIterator::new(large_int);
        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), Some(6));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(3));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn big_uint_0() {
        let zero = 0u64.to_biguint().unwrap();
        let mut digits = DigitIterator::new(zero);
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn big_uint_1() {
        let one = 1u64.to_biguint().unwrap();
        let mut digits = DigitIterator::new(one);
        assert_eq!(digits.next(), Some(1u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn big_uint_10() {
        let ten = 10u64.to_biguint().unwrap();
        let mut digits = DigitIterator::new(ten);
        assert_eq!(digits.next(), Some(0u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(1u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn big_uint_12345678() {
        let large_int = 12345678u64.to_biguint().unwrap();
        let mut digits = DigitIterator::new(large_int);
        assert_eq!(digits.next(), Some(8u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(7u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(6u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(5u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(4u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(3u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(2u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), Some(1u64.to_biguint().unwrap()));
        assert_eq!(digits.next(), None);
    }

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
