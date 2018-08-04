use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(100);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}

fn sum_of_squares(n: usize) -> usize {
    (1..=n).map(|x| x * x).sum()
}

fn square_of_sum(n: usize) -> usize {
    (1..=n).sum::<usize>().pow(2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(100), 25164150);
    }
}
