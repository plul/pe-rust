extern crate shared;

use shared::util::is_even;

struct Fib {
    x: (u64, u64),
}

impl Fib {
    fn new() -> Fib {
        Fib { x: (0, 1) }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.1)
    }
}

fn main() {
    let f = Fib::new();

    let result: u64 = f
        .take_while(|n| n < &4_000_000)
        .filter(|n| is_even(*n))
        .sum();

    println!("{}", result);
}
