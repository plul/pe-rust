use std::ops::{AddAssign, MulAssign};

pub fn integer_from_digit_slice<T: From<u8> + AddAssign + MulAssign>(digit_slice: &[u8]) -> T {
    let mut n: T = 0.into();
    for d in digit_slice {
        n *= 10.into();
        n += (*d).into();
    }
    n
}
