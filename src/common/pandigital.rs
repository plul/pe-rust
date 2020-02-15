use crate::common::radix::Radix;
use num::FromPrimitive;

pub trait Pandigital {
    fn is_pandigital_in_radix(&self, radix: u8) -> bool;

    fn is_n_pandigital_in_radix(&self, radix: u8) -> bool;

    fn is_exact_n_pandigital_in_radix(&self, radix: u8) -> bool;

    fn is_zeroless_pandigital_in_radix(&self, radix: u8) -> bool;

    fn is_zeroless_n_pandigital_in_radix(&self, radix: u8) -> bool;

    fn is_zeroless_exact_n_pandigital_in_radix(&self, radix: u8) -> bool;
}

impl<T> Pandigital for T
where
    T: Radix + PartialEq + FromPrimitive,
{
    fn is_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        digits.dedup();
        let expected: Vec<u8> = (0..radix).collect();
        digits == expected
    }

    fn is_n_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        digits.dedup();
        let expected: Vec<u8> = (0..radix).take(digits.len()).collect();
        digits == expected
    }

    fn is_exact_n_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        let expected: Vec<u8> = (0..radix).take(digits.len()).collect();
        digits == expected
    }

    fn is_zeroless_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        digits.dedup();
        let expected: Vec<u8> = (1..radix).collect();
        digits == expected
    }

    fn is_zeroless_n_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        digits.dedup();
        let expected: Vec<u8> = (1..radix).take(digits.len()).collect();
        digits == expected
    }

    fn is_zeroless_exact_n_pandigital_in_radix(&self, radix: u8) -> bool {
        assert!(radix > 0);

        // The value zero can never be a pandigital.
        if *self == FromPrimitive::from_u8(0).expect("0 must be convertible to type Self") {
            return false;
        }

        // In the unary base, everything except the value zero is pandigital.
        if radix == 1 {
            return true;
        }

        let mut digits = self.to_radix_le(u32::from(radix));
        digits.sort();
        let expected: Vec<u8> = (1..radix).take(digits.len()).collect();
        digits == expected
    }
}

#[test]
fn smallest_pandigitals() {
    assert!(2_u32.is_pandigital_in_radix(2));
    assert!(11_u32.is_pandigital_in_radix(3));
    assert!(75_u32.is_pandigital_in_radix(4));
    assert!(2177399_u32.is_pandigital_in_radix(8));
    assert!(1023456789_u32.is_pandigital_in_radix(10));
    assert!(754777787027_u64.is_pandigital_in_radix(12));
}

#[test]
fn too_small_to_be_pandigitals() {
    assert!(!(2_u32 - 1).is_pandigital_in_radix(2));
    assert!(!(11_u32 - 1).is_pandigital_in_radix(3));
    assert!(!(75_u32 - 1).is_pandigital_in_radix(4));
    assert!(!(2177399_u32 - 1).is_pandigital_in_radix(8));
    assert!(!(1023456789_u32 - 1).is_pandigital_in_radix(10));
    assert!(!(754777787027_u64 - 1).is_pandigital_in_radix(12));
}

#[test]
fn radix_1() {
    assert!(!0.is_pandigital_in_radix(1));
    assert!(1.is_pandigital_in_radix(1));
    assert!(2.is_pandigital_in_radix(1));
    assert!(3.is_pandigital_in_radix(1));
    assert!(4.is_pandigital_in_radix(1));
    assert!(5.is_pandigital_in_radix(1));
}

#[test]
fn radix_10() {
    assert!(1234567890.is_pandigital_in_radix(10));
    assert!(123456789000_u64.is_pandigital_in_radix(10));
    assert!(102345678999_u64.is_pandigital_in_radix(10));
    assert!(11223344556677889900_u64.is_pandigital_in_radix(10));

    assert!(!10.is_pandigital_in_radix(10));
    assert!(!210.is_pandigital_in_radix(10));
    assert!(!3210.is_pandigital_in_radix(10));
    assert!(!43210.is_pandigital_in_radix(10));

    assert!(!123456789.is_pandigital_in_radix(10));
}

#[test]
fn zeroless() {
    assert!(123456789.is_zeroless_pandigital_in_radix(10));
    assert!(381654729.is_zeroless_pandigital_in_radix(10));
    assert!(987654321.is_zeroless_pandigital_in_radix(10));
    assert!(12345678987654321_u64.is_zeroless_pandigital_in_radix(10));

    assert!(!1234568790.is_zeroless_pandigital_in_radix(10));
}

#[test]
fn n_pandigital() {
    assert!(!1.is_exact_n_pandigital_in_radix(10));
    assert!(1.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(10.is_exact_n_pandigital_in_radix(10));
    assert!(!10.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(210.is_exact_n_pandigital_in_radix(10));
    assert!(!210.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(!21.is_exact_n_pandigital_in_radix(10));
    assert!(21.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(!23451.is_exact_n_pandigital_in_radix(10));
    assert!(23451.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(203451.is_exact_n_pandigital_in_radix(10));
    assert!(!203451.is_zeroless_exact_n_pandigital_in_radix(10));
}

#[test]
fn basic_radix_10() {
    assert!(!123.is_pandigital_in_radix(10));
    assert!(!123.is_n_pandigital_in_radix(10));
    assert!(!123.is_exact_n_pandigital_in_radix(10));
    assert!(!123.is_zeroless_pandigital_in_radix(10));
    assert!(123.is_zeroless_n_pandigital_in_radix(10));
    assert!(123.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(!1023.is_pandigital_in_radix(10));
    assert!(1023.is_n_pandigital_in_radix(10));
    assert!(1023.is_exact_n_pandigital_in_radix(10));
    assert!(!1023.is_zeroless_pandigital_in_radix(10));
    assert!(!1023.is_zeroless_n_pandigital_in_radix(10));
    assert!(!1023.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(!112233.is_pandigital_in_radix(10));
    assert!(!112233.is_n_pandigital_in_radix(10));
    assert!(!112233.is_exact_n_pandigital_in_radix(10));
    assert!(!112233.is_zeroless_pandigital_in_radix(10));
    assert!(112233.is_zeroless_n_pandigital_in_radix(10));
    assert!(!112233.is_zeroless_exact_n_pandigital_in_radix(10));

    assert!(!11002233.is_pandigital_in_radix(10));
    assert!(11002233.is_n_pandigital_in_radix(10));
    assert!(!11002233.is_exact_n_pandigital_in_radix(10));
    assert!(!11002233.is_zeroless_pandigital_in_radix(10));
    assert!(!11002233.is_zeroless_n_pandigital_in_radix(10));
    assert!(!11002233.is_zeroless_exact_n_pandigital_in_radix(10));
}
