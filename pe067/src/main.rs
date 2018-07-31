extern crate pe018;

use pe018::problem;
use std::time::Instant;

fn main() {
    let t_0 = Instant::now();
    let result = problem("triangle.txt");
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(problem("triangle.txt"), 7273);
    }
}
