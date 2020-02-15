//! This is a solution to [Project Euler Problem 44](https://projecteuler.net/problem=44).
//!
//! This does not need an upper bound.

use std::fmt::Display;

use std::collections::HashSet;

// use num::bigint::BigUint;

#[inline]
fn is_odd(x: u64) -> bool {
    x & 1 == 1
}

#[inline]
fn is_even(x: u64) -> bool {
    x & 1 == 0
}

pub fn solve() -> impl Display {
    let mut cache: HashSet<u64> = HashSet::new();

    let mut diff_n = 0;
    loop {
        diff_n += 1;

        let diff = p(diff_n);
        cache.insert(diff);

        // Sum must be greater than the difference.
        let mut sum_n = diff_n;

        // Mark the point where the differences between p(n) and p(n+1) gets bigger than
        // diff.
        let mut mark: Option<u64> = None;

        let mut sum = p(sum_n);
        loop {
            // Need a stop condition!
            sum_n += 1;

            let next_sum = p(sum_n);
            if mark.is_none() && next_sum - sum > diff {
                mark = Some(next_sum);
            }
            sum = next_sum;

            cache.insert(sum);

            // Is there two pentagonal numbers with the given diff and sum?

            // If sum is even and difference is odd there can be no solution.
            if is_even(sum) && is_odd(diff) {
                continue;
            }

            // If sum is odd and difference is even there can be no solution.
            if is_odd(sum) && is_even(diff) {
                continue;
            }

            // Calculate what the two pentagonal numbers must be
            let p_j = (sum - diff) / 2;
            let p_k = (sum - diff) / 2 + diff;

            if let Some(m) = mark {
                if p_j >= m {
                    break;
                }
            }

            // Check if they are indeed pentagonal
            if cache.contains(&p_j) && cache.contains(&p_k) {
                return diff;
            }
        }
    }
}

/// Compute the pentagonal number p(n).
#[inline]
fn p(x: u64) -> u64 {
    x * (3 * x - 1) / 2
}

// #[test]
// fn test_is_pentagonal() {
//     assert!(is_pentagonal(1));
//     assert!(!is_pentagonal(2));
//     assert!(!is_pentagonal(3));
//     assert!(!is_pentagonal(4));
//     assert!(is_pentagonal(5));
//     assert!(!is_pentagonal(6));
//     assert!(is_pentagonal(12));
//     assert!(is_pentagonal(22));

//     for x in 1..=145 {
//         if [1, 5, 12, 22, 35, 51, 70, 92, 117, 145].contains(&x) {
//             assert!(is_pentagonal(x));
//         } else {
//             assert!(!is_pentagonal(x));
//         }
//     }
// }
