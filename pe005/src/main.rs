use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(20);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(upper: usize) -> usize {
    (1..)
        .map(|x| x * upper)
        .find(|&x| is_divisible_up_to(x, upper))
        .unwrap()
}

fn is_divisible_up_to(n: usize, upper: usize) -> bool {
    (1..upper).rev().all(|x| n % x == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem(20), 232792560);
    }
}
