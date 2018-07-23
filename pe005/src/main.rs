fn is_divisible_up_to(n: &u64, upper: &u64) -> bool {
    (1..*upper).rev().all(|x| n % x == 0)
}

fn main() {
    let upper: u64 = 20;

    let result = (1..)
        .map(|x| x * upper)
        .find(|x| is_divisible_up_to(x, &upper))
        .unwrap();

    println!("{}", result);
}
