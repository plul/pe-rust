fn sum_of_divisors(n: usize) -> usize {
    (1..=n / 2).filter(|&x| n % x == 0).sum()
}

fn main() {
    let result: usize = (1usize..10000)
        .filter_map(|n| {
            let s = sum_of_divisors(n);
            if s < n && n == sum_of_divisors(s) {
                return Some(s + n);
            }
            None
        })
        .sum();

    println!("{}", result);
}
