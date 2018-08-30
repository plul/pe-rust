//! This is a solution to [Project Euler Problem 44](https://projecteuler.net/problem=44).
//!
//! This does not need an upper bound.
//! It will search until the difference between p(i) and p(i - 1) is greater than the best candidate found so far.

use std::fmt::Display;

pub fn solve() -> impl Display {
    let mut candidate = u32::max_value();

    for i in 2.. {
        if p(i) - p(i - 1) > candidate {
            return candidate;
        }

        for j in 1..i {
            let diff = p(i) - p(j);
            if diff >= candidate {
                continue;
            }

            let sum = p(i) + p(j);
            if p_inverse(diff).fract() == 0.0 && p_inverse(sum).fract() == 0.0 {
                candidate = diff;
            }
        }
    }

    panic!("No candidate found");
}

/// Compute the pentagonal number p(n).
fn p(x: u32) -> u32 {
    x * (3 * x - 1) / 2
}

/// Convert from a pentagonal number p(n) to n.
fn p_inverse(x: u32) -> f64 {
    let a = 24 * x + 1;
    let a = f64::from(a).sqrt() + 1.0;
    let a = a / 6.0;
    a
}

#[test]
fn test_is_pentagonal() {
    for x in 1..100 {
        assert!(p_inverse(p(x)).fract() == 0.0);
    }
    for x in 1..100 {
        assert!(p_inverse(p(x) + 1).fract() != 0.0);
        assert!(p_inverse(p(x) - 1).fract() != 0.0);
    }
}
