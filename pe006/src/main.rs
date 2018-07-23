fn sum_of_squares(n: &u64) -> u64 {
    (1..=*n).map(|x| x * x).sum()
}

fn square_of_sum(n: &u64) -> u64 {
    let sum: u64 = (1..=*n).sum();
    sum * sum
}

fn main() {
    let n = 100;
    let result: u64 = square_of_sum(&n) - sum_of_squares(&n);

    println!("{}", result);
}
