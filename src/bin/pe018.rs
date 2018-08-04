extern crate project_euler;

use project_euler::maximum_path_sum;
use std::time::Instant;

const DATA: &[u8] = include_bytes!("pe018-data.txt");

fn main() {
    let t_0 = Instant::now();
    let result = problem();
    let t_1 = Instant::now();

    println!("Result: {}", result);
    println!("Time:   {:?}", t_1 - t_0);
}

fn problem() -> usize {
    let data: &str = std::str::from_utf8(DATA).unwrap();
    maximum_path_sum::problem(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_solution() {
        assert_eq!(problem(), 1074);
    }
}
