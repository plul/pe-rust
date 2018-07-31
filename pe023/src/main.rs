use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let mut abundant_numbers: Vec<usize> = Vec::new();

    // table[n] indicates whether n is expressible as the sum of two abundant numbers.
    let mut table = vec![false; 28123];

    let mut result = 0;

    for n in 1..28123 {
        if !table[n] {
            // n cannot be expressed as the sum of two abundant numbers.
            result += n;
        }

        if divisors(n).sum::<usize>() > n {
            // n is abundant.
            abundant_numbers.push(n);

            // Match n with any previously found abundant number, and mark the sums as expressible by the sum of two abundant numbers.
            for an in abundant_numbers.iter() {
                let s = an + n;
                if s >= table.len() {
                    break;
                }
                table[s] = true;
            }
        }
    }

    result
}

fn divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n / 2).filter(move |x| n % x == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divisors_of_28() {
        assert_eq!(divisors(28).sum::<usize>(), 28);
    }

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem(), 4179871);
    }
}
