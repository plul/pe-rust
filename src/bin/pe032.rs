//! This is a solution to [Project Euler Problem 32](https://projecteuler.net/problem=32).

use std::time::Instant;
use std::collections::HashSet;

struct Permutations<T: Clone> {
    collection: Vec<T>,
    p: usize,
    possibilities: usize,
}

impl<T: Clone> Permutations<T> {
    fn new(collection: Vec<T>) -> Permutations<T> {
        let possibilities = Self::factorial(collection.len());
        Permutations {
            collection,
            p: 0,
            possibilities,
        }
    }

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    fn permutation(&self, mut target: usize) -> Vec<T> {
        let mut c = self.collection.clone();
        let mut v = Vec::<T>::with_capacity(c.len());

        while !c.is_empty() {
            let l = c.len();
            let f = Self::factorial(l);

            let i = l * target / f;
            v.push(c.remove(i));

            target %= f / l;
        }

        v
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p == self.possibilities {
            None
        } else {
            let permutation = self.permutation(self.p);
            self.p += 1;
            Some(permutation)
        }
    }
}

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let digits: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutations = Permutations::new(digits);

    let mut discoveries = HashSet::new();

    for p in permutations {
            // Playing around with the numbers for a bit reveals that the pattern must be either:
            // a * bbb == cccc
            // or
            // aa * bb == cccc
            let (lhs, rhs) = p.split_at(5);
            for j in 1..=2 {
                let (multiplicant, multiplier) = lhs.split_at(j);

                let multiplicant = integer_from_digits(multiplicant);
                let multiplier = integer_from_digits(multiplier);
                let rhs = integer_from_digits(rhs);

                if multiplicant * multiplier == rhs {
                    discoveries.insert(rhs);
                }
        }
    }

    discoveries.iter().sum()
}

fn integer_from_digits(digits: &[u8]) -> usize {
    let mut n = 0;
    for d in digits {
        n *= 10;
        n += usize::from(*d);
    }
    n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 45228)
    }
}
