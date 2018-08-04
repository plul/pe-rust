use itertools::Itertools;
use std::iter;

pub struct Factor {
    pub n: usize,
    pub exponent: usize,
}

pub struct PrimeFactorization {
    pub prime_factors: Vec<Factor>,
}

impl PrimeFactorization {
    pub fn factor(mut n: usize) -> PrimeFactorization {
        let mut prime_factors = Vec::<usize>::new();

        let mut d = 2;
        while n > 1 {
            if n % d == 0 {
                n /= d;
                prime_factors.push(d);
            } else {
                d += 1;
            }
        }

        let prime_factors_grouped = prime_factors
            .into_iter()
            .group_by(|x| *x)
            .into_iter()
            .map(|(k, group)| Factor {
                n: k,
                exponent: group.count(),
            })
            .collect();

        PrimeFactorization {
            prime_factors: prime_factors_grouped,
        }
    }

    pub fn product(&self) -> usize {
        self.prime_factors
            .iter()
            .flat_map(|factor| iter::repeat(factor.n).take(factor.exponent))
            .product()
    }
}
