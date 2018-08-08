extern crate num;

use num::bigint::ToBigUint;

/// A generic trait for converting a value to a `Vec<u8>`.
///
/// Digits are presented from least significant to most significant digit.
pub trait ToDigits {
    /// Converts the value of `self` to a `Vec<u8>`.
    fn to_digits(&self) -> Vec<u8>;
}

impl<T> ToDigits for T
where
    T: ToBigUint,
{
    fn to_digits(&self) -> Vec<u8> {
        self.to_biguint()
            .expect("Conversion to BigUint failed")
            .to_radix_le(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_0() {
        let mut digits_into_iter = 0_u64.to_digits().into_iter();
        assert_eq!(digits_into_iter.next(), Some(0));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_1() {
        let mut digits_into_iter = 1_u64.to_digits().into_iter();
        assert_eq!(digits_into_iter.next(), Some(1));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_10() {
        let mut digits_into_iter = 10_u64.to_digits().into_iter();
        assert_eq!(digits_into_iter.next(), Some(0));
        assert_eq!(digits_into_iter.next(), Some(1));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_12345678() {
        let digits: Vec<u8> = 12345678_u64.to_digits();
        assert_eq!(digits, [8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
