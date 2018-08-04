use std::ops::{DivAssign, Rem};

pub struct DigitIterator<N: From<u8> + Clone + PartialEq + DivAssign + Rem<Output = N>> {
    n: N,
}

impl<N: From<u8> + Clone + PartialEq + DivAssign + Rem<Output = N>> DigitIterator<N> {
    pub fn new(n: N) -> DigitIterator<N> {
        DigitIterator { n }
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
    use num::bigint::ToBigUint;

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
}
