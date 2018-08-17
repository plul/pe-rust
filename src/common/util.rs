use std::ops::{AddAssign, MulAssign};

pub fn integer_from_digit_slice<'a, T, U>(digits: U) -> T
where
    T: From<u8> + AddAssign + MulAssign,
    U: Iterator<Item = &'a u8>,
{
    let mut n: T = 0.into();
    for d in digits {
        n *= 10.into();
        n += (*d).into();
    }
    n
}
