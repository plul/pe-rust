fn first_factor(n: u64) -> u64 {
    for d in 2..n {
        if n % d == 0 {
            return d;
        }
    }

    n
}

fn largest_prime_factor(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut lpf = 1u64;

    while n > 1 {
        let ff = first_factor(n);
        if ff > lpf {
            lpf = ff;
        }
        n /= ff;
    }

    Some(lpf)
}

fn main() {
    let n = 600851475143;

    if let Some(lpf) = largest_prime_factor(n) {
        println!("{}", lpf);
    };
}
