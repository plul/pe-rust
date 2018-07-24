use std::ops::BitAnd;

pub fn is_even<N: From<u8> + PartialEq + BitAnd<Output = N>>(n: N) -> bool {
    n & 1.into() == 0.into()
}
