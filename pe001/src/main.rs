use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem(1000);
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem(x: u64) -> u64 {
    (1..x).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem(1000), 233168);
    }
}
