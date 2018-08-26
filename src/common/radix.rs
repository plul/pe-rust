extern crate num;

use num::bigint::ToBigUint;

/// A generic trait for converting a value to a `Vec<u8>`.
///
/// This uses the corresponding functions on num::BigUint, so
/// the radix must be in the range [2..256].
pub trait Radix {
    /// Represent self as digits in a given base (radix).
    ///
    /// Little endian.
    fn to_radix_le(&self, radix: u32) -> Vec<u8>;

    /// Represent self as digits in a given base (radix).
    ///
    /// Big endian.
    fn to_radix_be(&self, radix: u32) -> Vec<u8>;
}

impl<T> Radix for T
where
    T: ToBigUint,
{
    fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        self.to_biguint()
            .expect("Conversion to BigUint failed")
            .to_radix_le(radix)
    }

    fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        self.to_biguint()
            .expect("Conversion to BigUint failed")
            .to_radix_be(radix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_0() {
        let mut digits_into_iter = 0_u64.to_radix_le(10).into_iter();
        assert_eq!(digits_into_iter.next(), Some(0));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_1() {
        let mut digits_into_iter = 1_u64.to_radix_le(10).into_iter();
        assert_eq!(digits_into_iter.next(), Some(1));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_10() {
        let mut digits_into_iter = 10_u64.to_radix_le(10).into_iter();
        assert_eq!(digits_into_iter.next(), Some(0));
        assert_eq!(digits_into_iter.next(), Some(1));
        assert_eq!(digits_into_iter.next(), None);
    }

    #[test]
    fn u64_12345678() {
        let digits: Vec<u8> = 12345678_u64.to_radix_le(10);
        assert_eq!(digits, [8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
